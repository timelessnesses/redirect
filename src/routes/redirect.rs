use actix_web;
use log;
use tokio;
use uuid;
fn report_no_database() -> actix_web::HttpResponse {
    log::error!("No database connection is found!");
    actix_web::HttpResponse::InternalServerError()
        .body("No database connection available! Please restart your server!")
}

fn report_not_serious_add(e: String) {
    return log::error!(
        "Error at /add checking if there's ID {}\nNOTE: this is most likely harmless",
        e
    );
}

#[actix_web::get("/add")]
pub async fn add(
    params: actix_web::web::Query<crate::routes::types::AddParameters>,
    app: actix_web::web::Data<crate::routes::types::States>,
    req: actix_web::HttpRequest,
) -> impl actix_web::Responder {
    log::info!("I got called! (add)");
    let pg = app.postgres_db.as_ref();
    let sqlite3 = app.sqlite3_db.as_ref();

    let url = params.url.to_owned();

    match (app.postgres_db.is_some(), app.sqlite3_db.is_some()) {
        (true, false) => {
            let id = pg
                .as_ref()
                .unwrap()
                .query("SELECT * FROM redirect WHERE url = $1", &[&url])
                .await;

            match id {
                Ok(row) => {
                    if row.len() == 1 {
                        let id: &str = row[0].get("id");
                        return actix_web::HttpResponse::Ok().body(format!(
                            "{}://{}/{}",
                            req.connection_info().scheme(),
                            req.connection_info().host(),
                            id
                        ));
                    }
                }
                Err(e) => report_not_serious_add(e.to_string()),
            }
        }
        (false, true) => {
            match sqlite3
                .as_ref()
                .unwrap()
                .call(move |c| {
                    return c.query_row("SELECT id FROM redirect WHERE url = ?", [&url], |r| {
                        let id: Result<String, rusqlite::Error> = r.get(0);
                        return id;
                    });
                })
                .await
            {
                Ok(id) => {
                    return actix_web::HttpResponse::Ok().body(format!(
                        "{}://{}/{}",
                        req.connection_info().scheme(),
                        req.connection_info().host(),
                        id
                    ))
                }
                Err(e) => report_not_serious_add(e.to_string()),
            }
        }
        _ => {
            log::error!("No database connection is found!");
            return actix_web::HttpResponse::InternalServerError()
                .body("Something went terribly wrong witht the database!");
        }
    }

    let id = uuid::Uuid::new_v4();
    let url = params.url.to_owned();

    match (app.postgres_db.is_some(), app.sqlite3_db.is_some()) {
        (true, false) => {
            let db = pg.as_ref().unwrap();
            match db
                .execute(
                    "INSERT INTO redirect(url, accessed, id) VALUES ($1, $2, $3)",
                    &[&url, &(0 as i64), &id.to_string().as_str()],
                )
                .await
            {
                Ok(_) => actix_web::HttpResponse::Ok().body(format!(
                    "{}://{}/{}",
                    req.connection_info().scheme(),
                    req.connection_info().host(),
                    id
                )),
                Err(e) => {
                    log::error!("Error at /add while adding the url: {}", e);
                    actix_web::HttpResponse::InternalServerError()
                        .body("Cannot add your URL to redirect database!")
                }
            }
        }
        (false, true) => {
            let db = sqlite3.as_ref().unwrap();
            match db
                .call(move |c| {
                    c.execute(
                        "INSERT INTO redirect(url, accessed, id) VALUES (?, ?, ?)",
                        [&url, "0", (id.to_string().as_str())],
                    )
                })
                .await
            {
                Ok(_) => actix_web::HttpResponse::Ok().body(format!(
                    "{}://{}/{}",
                    req.connection_info().scheme(),
                    req.connection_info().host(),
                    id
                )),
                Err(e) => {
                    log::error!("Error at /add while adding the url: {}", e);
                    actix_web::HttpResponse::InternalServerError()
                        .body("Cannot add your URL to redirect database!")
                }
            }
        }
        _ => report_no_database(),
    }
}

#[actix_web::get("/{id}")]
pub async fn get(
    id: actix_web::web::Path<String>,
    app: actix_web::web::Data<crate::routes::types::States>,
) -> impl actix_web::Responder {
    log::info!("I got called! (get)");
    let pg = app.postgres_db.as_ref();
    let sqlite3 = app.sqlite3_db.as_ref();

    match (app.postgres_db.is_some(), app.sqlite3_db.is_some()) {
        (true, false) => {
            let db = pg.as_ref().unwrap();
            let cook = db
                .query(
                    "SELECT url FROM redirect WHERE id = $1",
                    &[&id.to_owned().as_str()],
                )
                .await;
            match cook {
                Ok(row) => {
                    if row.len() == 0 {
                        return actix_web::HttpResponse::BadRequest()
                            .body("No URLs associated with that ID you provided!");
                    }
                    let url: &str = row[0].get(0);
                    match db
                        .execute(
                            "UPDATE redirect SET accessed = accessed + 1 WHERE id = $1",
                            &[&id.to_owned().as_str()],
                        )
                        .await
                    {
                        Err(e) => {
                            return {
                                log::error!("Cannot update access count! What's going on?: {}", e);
                                actix_web::HttpResponse::InternalServerError().body("Cannot update accessed count! Are you sure the database is connected and alive?")
                            }
                        }
                        _ => {}
                    }
                    return actix_web::HttpResponse::TemporaryRedirect()
                        .append_header(("location", url))
                        .body(format!(
                            "Are you getting redirected? If not, Click this link! -> {}",
                            url
                        ));
                }
                Err(e) => {
                    log::warn!("That ID isn't in the database!: {}", e);
                    actix_web::HttpResponse::InternalServerError()
                        .body("No associated IDs matched with in the database!")
                }
            }
        }
        (false, true) => {
            let db = sqlite3.as_ref().unwrap();
            match db
                .call(
                    move |c| match c.prepare("SELECT url FROM redirect WHERE id = ?") {
                        Ok(mut stmt) => {
                            return stmt.query_row([&id.to_owned().as_str()], |r| {
                                let url: Result<String, rusqlite::Error> = r.get(0);
                                url
                            })
                        }
                        Err(e) => return Err(e),
                    },
                )
                .await
            {
                Ok(url) => {
                    let url = url.as_str();
                    return actix_web::HttpResponse::TemporaryRedirect()
                        .append_header(("location", url))
                        .body(format!(
                            "Are you getting redirected? If not, Click this link! -> {}",
                            url
                        ));
                }
                Err(e) => {
                    log::warn!("That ID isn't in the database!: {}", e);
                    actix_web::HttpResponse::NotFound()
                        .body("No associated IDs matched with in the database!")
                }
            }
        }
        _ => report_no_database(),
    }
}

#[actix_web::get("/update/{id}")]
pub async fn update(
    id: actix_web::web::Path<String>,
    app: actix_web::web::Data<crate::routes::types::States>,
    url: actix_web::web::Query<crate::routes::types::UpdateParamaters>,
) -> impl actix_web::Responder {
    let pg = app.postgres_db.as_ref();
    let sqlite3 = app.sqlite3_db.as_ref();

    match (pg.is_some(), sqlite3.is_some()) {
        (true, false) => {
            let db = pg.as_ref().unwrap();
            match db
                .query(
                    "SELECT * FROM redirect WHERE id = $1",
                    &[&id.to_owned().as_str()],
                )
                .await
            {
                Ok(rows) => {
                    if rows.len() == 0 {
                        return actix_web::HttpResponse::BadRequest()
                            .body("ID isn't in the recorded ID database!");
                    }
                    match db
                        .execute(
                            "UPDATE redirect SET url = $1, accessed = 0 WHERE id = $2",
                            &[&url.url.as_str(), &id.as_str()],
                        )
                        .await
                    {
                        Ok(_) => return actix_web::HttpResponse::Ok().body("Success!"),
                        Err(e) => {
                            log::error!("Cannot update URL! Is something went wrong?: {}", e);
                            return actix_web::HttpResponse::InternalServerError()
                                .body("Cannot update your redirect URL!");
                        }
                    }
                }
                Err(e) => {
                    log::error!("Cannot check ID! What's going on?: {}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Cannot check if ID is exists!");
                }
            }
        }
        (false, true) => {
            let db = sqlite3.as_ref().unwrap();
            match db
                .call(move |c| {
                    c.query_row(
                        "SELECT id FROM redirect WHERE id = ?",
                        [&id.to_owned().as_str()],
                        |r| {
                            let id: Result<String, rusqlite::Error> = r.get(0);
                            id
                        },
                    )
                })
                .await
            {
                Ok(id) => {
                    match db
                        .call(move |c| {
                            c.execute("UPDATE redirect SET url = ? WHERE id = ?", [&url.url, &id])
                        })
                        .await
                    {
                        Ok(_) => return actix_web::HttpResponse::Ok().body("Success!"),
                        Err(e) => {
                            log::error!("Cannot update the URL! What's going on?: {}", e);
                            return actix_web::HttpResponse::InternalServerError()
                                .body("Cannot update the URL!");
                        }
                    }
                }
                Err(e) => {
                    log::error!("Cannot check ID! What's going on here?: {}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Cannot check if ID is exists!");
                }
            }
        }
        _ => report_no_database(),
    }
}

#[actix_web::get("/stat/{id}")]
pub async fn stat(
    id: actix_web::web::Path<String>,
    app: actix_web::web::Data<crate::routes::types::States>,
) -> impl actix_web::Responder {
    let postgres = app.postgres_db.as_ref();
    let sqlite3 = app.sqlite3_db.as_ref();

    match (postgres.is_some(), sqlite3.is_some()) {
        (true, false) => {
            let db = postgres.as_ref().unwrap();
            match db
                .query(
                    "SELECT * FROM redirect WHERE id = $1",
                    &[&id.to_owned().as_str()],
                )
                .await
            {
                Ok(rows) => {
                    if rows.len() == 1 {
                        let url: String = rows[0].get("url");
                        let accessed: i64 = rows[0].get("accessed");
                        return actix_web::HttpResponse::Ok().body(format!(
                            "Stats:\nURL: {}\nID: {}\nAccessed: {} times!",
                            url, id, accessed
                        ));
                    } else {
                        return actix_web::HttpResponse::BadRequest()
                            .body("No information with that ID found!");
                    }
                }
                Err(e) => {
                    log::error!("Something went wrong. It maybe harmless. {}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Something went wrong when requesting informations from database!");
                }
            }
        }
        (false, true) => {
            let db = sqlite3.as_ref().unwrap();
            let id1 = id.clone();
            match db
                .call(
                    move |c| match c.prepare("SELECT url, accessed FROM redirect WHERE id = ?") {
                        Ok(mut stmt) => {
                            match stmt.query_map([&id.to_owned().as_str()], |r| {
                                Ok(crate::routes::types::StatsRequestedData {
                                    url: r.get(0).unwrap(),
                                    accessed: r.get(1).unwrap(),
                                })
                            }) {
                                Ok(mapped) => {
                                    return Ok::<_, rusqlite::Error>(
                                        mapped
                                            .collect::<Result<
                                                Vec<crate::routes::types::StatsRequestedData>,
                                                rusqlite::Error,
                                            >>()
                                            .unwrap(),
                                    )
                                }
                                Err(e) => return Err::<_, rusqlite::Error>(e),
                            }
                        }
                        Err(e) => return Err::<_, rusqlite::Error>(e),
                    },
                )
                .await
            {
                Ok(data) => {
                    let url = &data[0].url;
                    let accessed = &data[0].accessed;
                    return actix_web::HttpResponse::Ok().body(format!(
                        "Stats:\nURL: {}\nID: {}\nAccessed: {} times!",
                        url, id1, accessed
                    ));
                }
                Err(e) => {
                    log::error!("Something really fucked up. {}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Something went wrong when fetching stat for that ID!");
                }
            }
        }
        _ => report_no_database(),
    }
}

#[actix_web::get("/remove/{id}")]
pub async fn remove(
    id: actix_web::web::Path<String>,
    app: actix_web::web::Data<crate::routes::types::States>,
) -> impl actix_web::Responder {
    let postgres = app.postgres_db.as_ref();
    let sqlite3 = app.sqlite3_db.as_ref();

    match (postgres.is_some(), sqlite3.is_some()) {
        (true, false) => {
            let db = postgres.as_ref().unwrap();
            match db
                .query(
                    "SELECT url FROM redirect WHERE id = $1",
                    &[&id.to_owned().as_str()],
                )
                .await
            {
                Ok(rows) => {
                    if rows.len() == 1 {
                        match db
                            .query(
                                "DELETE FROM redirect WHERE id = $1",
                                &[&id.to_owned().as_str()],
                            )
                            .await
                        {
                            Ok(_) => return actix_web::HttpResponse::Ok().body("Success!"),
                            Err(e) => {
                                log::error!("Cannot delete. {}", e);
                                return actix_web::HttpResponse::InternalServerError()
                                    .body("Cannot delete this ID off from database!");
                            }
                        }
                    } else {
                        return actix_web::HttpResponse::BadRequest().body("No ID found!");
                    }
                }
                Err(e) => {
                    log::error!("Cannot check if the ID exists before delete!: {}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Cannot check if ID exists!");
                }
            }
        }
        (false, true) => {
            let db = sqlite3.as_ref().unwrap();
            let id1 = id.clone();
            match db
                .call(move |c| {
                    c.query_row(
                        "SELECT url FROM redirect WHERE id = ?",
                        [&id.to_owned().as_str()],
                        |r| {
                            let url: Result<String, rusqlite::Error> = r.get(0);
                            return url;
                        },
                    )
                })
                .await
            {
                Ok(_) => {
                    match db
                        .call(move |c| {
                            c.execute(
                                "DELETE FROM redirect WHERE id = ?",
                                [&id1.to_owned().as_str()],
                            )
                        })
                        .await
                    {
                        Ok(_) => return actix_web::HttpResponse::Ok().body("Success!"),
                        Err(e) => {
                            log::error!("Cannot delete. {}", e);
                            return actix_web::HttpResponse::InternalServerError()
                                .body("Cannot delete this ID off from database!");
                        }
                    }
                }
                Err(e) => {
                    log::error!("Cannot check if ID is there or ID is not exists!: {}", e);
                    return actix_web::HttpResponse::BadRequest().body("Not exists!");
                }
            }
        }
        _ => report_no_database(),
    }
}

#[actix_web::get("/list")]
pub async fn listing(
    app: actix_web::web::Data<crate::routes::types::States>,
) -> impl actix_web::Responder {
    match (app.postgres_db.is_some(), app.sqlite3_db.is_some()) {
        (true, false) => {
            match app
                .postgres_db
                .as_ref()
                .as_ref()
                .unwrap()
                .query("SELECT * FROM redirect", &[])
                .await
            {
                Ok(rows) => {
                    let (tx, mut rx) = tokio::sync::mpsc::channel::<
                        Result<actix_web::web::Bytes, actix_web::Error>,
                    >(10);
                    actix_web::rt::spawn(async move {
                        match tx
                            .send(Ok(actix_web::web::Bytes::from(
                                "List of redirect URLs!\n".to_owned().into_bytes(),
                            )))
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Something went wrong when streaming data!: {}", e);
                                return;
                            }
                        }

                        if rows.len() == 0 {
                            match tx
                                .send(Ok(actix_web::web::Bytes::from(
                                    "None!".to_owned().into_bytes(),
                                )))
                                .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    log::error!("Something went wrong when streaming data!: {}", e);
                                    return;
                                }
                            }
                        }

                        for row in rows {
                            let id: String = row.get("id");
                            let url: String = row.get("url");
                            let accessed: i64 = row.get("accessed");
                            let data = format!("ID: {}, URL: {}, Accessed: {}", id, url, accessed)
                                .into_bytes();
                            match tx.send(Ok(actix_web::web::Bytes::from(data))).await {
                                Ok(_) => {}
                                Err(e) => {
                                    log::error!("Something went wrong when streaming data!: {}", e);
                                    break;
                                }
                            }
                        }
                    });
                    return actix_web::HttpResponse::Ok().streaming(async_stream::stream! {
                        while let Some(item) = rx.recv().await {
                            match item {
                                Ok(data) => yield Ok(data),
                                Err(err) => yield Err(actix_web::Error::from(err))
                            }
                        }
                    });
                }
                Err(e) => {
                    log::error!("Cannot get list of redirect URLs! What's going on?: {}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Cannot get list of redirect URLs!");
                }
            }
        }
        (false, true) => {
            match app
                .sqlite3_db
                .as_ref()
                .as_ref()
                .unwrap()
                .call(|c| {
                    match c.prepare("SELECT id,url,accessed FROM redirect") {
                        Ok(mut stmt) => {
                            return Ok(stmt
                                .query_map([], |r| {
                                    Ok(crate::routes::types::ListingRequestedData {
                                        id: r.get(0).unwrap(),
                                        url: r.get(1).unwrap(),
                                        accessed: r.get(2).unwrap(),
                                    })
                                })
                                .unwrap()
                                .collect::<Result<
                                    Vec<crate::routes::types::ListingRequestedData>,
                                    rusqlite::Error,
                                >>()
                                .unwrap())
                        }
                        Err(e) => return Err(e),
                    }
                })
                .await
            {
                Ok(rows) => {
                    let (tx, mut rx) = tokio::sync::mpsc::channel::<
                        Result<actix_web::web::Bytes, actix_web::Error>,
                    >(10);
                    actix_web::rt::spawn(async move {
                        match tx
                            .send(Ok(actix_web::web::Bytes::from(
                                "List of redirect URLs!\n".to_owned().into_bytes(),
                            )))
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("Something went wrong when streaming data!: {}", e);
                                return;
                            }
                        }

                        if rows.len() == 0 {
                            match tx
                                .send(Ok(actix_web::web::Bytes::from(
                                    "None!".to_owned().into_bytes(),
                                )))
                                .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    log::error!("Something went wrong when streaming data!: {}", e);
                                    return;
                                }
                            }
                        }

                        for row in rows {
                            let id: String = row.id;
                            let url: String = row.url;
                            let accessed: i64 = row.accessed;
                            let data = format!("ID: {}, URL: {}, Accessed: {}", id, url, accessed)
                                .into_bytes();
                            match tx.send(Ok(actix_web::web::Bytes::from(data))).await {
                                Ok(_) => {}
                                Err(e) => {
                                    log::error!("Something went wrong when streaming data!: {}", e);
                                    break;
                                }
                            }
                        }
                    });
                    return actix_web::HttpResponse::Ok().streaming(async_stream::stream! {
                        while let Some(item) = rx.recv().await {
                            match item {
                                Ok(data) => yield Ok(data),
                                Err(err) => yield Err(actix_web::Error::from(err))
                            }
                        }
                    });
                }
                Err(e) => {
                    log::error!("Cannot get list of redirect URLs! What's going on?: {}", e);
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Cannot get list of redirect URLs!");
                }
            }
        }
        _ => report_no_database(),
    }
}

#[actix_web::get("/stream_test")]
pub async fn stream_test(
    query: actix_web::web::Query<crate::routes::types::StreamingTest>,
) -> impl actix_web::Responder {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Result<actix_web::web::Bytes, actix_web::Error>>(
        query.output_amount.try_into().unwrap(),
    );
    actix_web::rt::spawn(async move {
        match tx
            .send(Ok(actix_web::web::Bytes::from(
                "Starting!".to_owned().into_bytes(),
            )))
            .await
        {
            Ok(_) => {}
            Err(e) => {
                log::error!("TESTING: stream test fails: {}", e);
                return;
            }
        }
        for i in 0..query.till {
            let x = i.to_string() + "\n";
            match tx.send(Ok(actix_web::web::Bytes::from(x))).await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("TESTING: stream test fails: {}", e);
                    break;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(
                query.wait.try_into().unwrap(),
            ))
            .await;
        }
    });
    return actix_web::HttpResponse::Ok().streaming(async_stream::stream! {
        while let Some(item) = rx.recv().await {
            match item {
                Ok(data) => yield Ok(data),
                Err(err) => yield Err(actix_web::Error::from(err))
            }
        }
    });
}

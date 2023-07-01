use actix_web;
use uuid;
use log;

#[actix_web::get("/add")]
pub async fn add(params: actix_web::web::Query<crate::routes::types::AddParameters>, app: actix_web::web::Data<crate::routes::types::States>, req: actix_web::HttpRequest) -> impl actix_web::Responder {
    log::info!("I got called! (add)");
    let url = params.url.to_owned();

    let pg = app.postgres_db.as_ref();
    let sqlite3 = app.sqlite3_db.as_ref();

    match (app.postgres_db.is_some(), app.sqlite3_db.is_some()) {
        (true, false) => {
            let id = pg.as_ref().unwrap().query_one("SELECT * FROM redirect WHERE url = $1", &[&url]).await;
            match id {
                Ok(row) => {
                    if row.len() != 0 {
                        let id: &str = row.get("url");
                        return actix_web::HttpResponse::Ok().body(format!("{}://{}/{}",req.connection_info().scheme(), req.connection_info().host(), id))
                    }
                },
                Err(_) => return actix_web::HttpResponse::InternalServerError().body("Cannot check for existing URLs!")
            }
        },
        (false, true) => {
            match sqlite3.as_ref().unwrap().call(move |c| {
                return c.query_row("SELECT * FROM redirect WHERE url = ?", [&url], |r| {
                    let id: Result<String, rusqlite::Error> = r.get(0);
                    return id
                });
            }).await {
                Ok(id) => {
                    return actix_web::HttpResponse::Ok().body(format!("{}://{}/{}",req.connection_info().scheme(), req.connection_info().host(), id))
                },
                Err(_) => return actix_web::HttpResponse::InternalServerError().body("Cannot check for existing URLs!")
            }
        },
        _ => {}
    }

    let id = uuid::Uuid::new_v4();

    match (app.postgres_db.is_some(), app.sqlite3_db.is_some()) {
        (true, false) => {
            let db = pg.as_ref().unwrap();
            match db.execute("INSERT INTO redirect(url, accessed, id) VALUES ($1, $2, $3)", &[&url, &(0 as i32), &id.to_string().as_str()]).await {
                Ok(_) => actix_web::HttpResponse::Ok().body(format!("{}://{}/{}",req.connection_info().scheme(), req.connection_info().host(), id)),
                Err(e) => {
                    log::error!("{}",e);
                    actix_web::HttpResponse::InternalServerError().body("Cannot add your URL to redirect database!")
                }
            }
        },
        (false, true) => {
            let db = sqlite3.as_ref().unwrap();
            match db.call(move |c| {
                c.execute("INSERT INTO redirect(url, accessed, id) VALUES (?, ?, ?)", [&url, "0", (id.to_string().as_str())])
            }).await {
                Ok(_) => actix_web::HttpResponse::Ok().body(format!("{}://{}/{}",req.connection_info().scheme(), req.connection_info().host(), id)),
                Err(_) => actix_web::HttpResponse::InternalServerError().body("Cannot add your URL to redirect database!")

            }
        },
        _ => actix_web::HttpResponse::InternalServerError().body("No database connection available! Please restart your server!")
    }
}

#[actix_web::get("/{id}")]
async fn get(id: actix_web::web::Path<String>, app: actix_web::web::Data<crate::routes::types::States>) -> impl actix_web::Responder {
    log::info!("I got called! (get)");
    let pg = app.postgres_db.as_ref();
    let sqlite3 = app.sqlite3_db.as_ref();

    match (app.postgres_db.is_some(), app.sqlite3_db.is_some()) {
        (true, false) => {
            let db = pg.as_ref().unwrap();
            let cook = db.query_one("SELECT url,accessed FROM redirect WHERE id = $1", &[&id.to_owned().as_str()]).await;
            match cook {
                Ok(row) => {
                    let mut accessed: i64 = row.get(1);
                    let url: &str = row.get(0);
                    accessed += 1;
                    match db.execute("UPDATE redirect(accessed) VALUES ($1) WHERE id = $2", &[&accessed, &id.to_owned().as_str()]).await {
                        Err(_) => return actix_web::HttpResponse::InternalServerError().body("Cannot update accessed count! Are you sure the database is connected and alive?"),
                        _ => {}
                    }
                    return actix_web::HttpResponse::TemporaryRedirect().append_header(("location", url)).body(format!("Are you getting redirected? If not, Click this link! -> {}", url))
                },
                Err(_) => {
                    actix_web::HttpResponse::NotFound().body("No associated IDs matched with in the database!")
                }
            }
        },
        (false, true) => {
            let db = sqlite3.as_ref().unwrap();
            match db.call(move |c| {
                match c.prepare("SELECT * FROM redirect WHERE id = ?") {
                    Ok(mut stmt) => {
                        return stmt.query_row([&id.to_owned().as_str()], |r| {
                            let url: Result<String, rusqlite::Error> = r.get(0);
                            url
                        })
                    },
                    Err(e) => {
                        return Err(e)
                    }
                }
            }).await {
                Ok(url) => {
                    let url = url.as_str();
                    return actix_web::HttpResponse::TemporaryRedirect().append_header(("location", url)).body(format!("Are you getting redirected? If not, Click this link! -> {}", url))
                },
                Err(_) => actix_web::HttpResponse::NotFound().body("No associated IDs matched with in the database!")
            }
        },
        _ => actix_web::HttpResponse::InternalServerError().body("No database connection available! Please restart your server!"),
    }

}

// #[actix_web::get("/update/{id}")]
// async fn update(id: actix_web::web::Path<String>, app: actix_web::web::Data<crate::routes::types::States>) -> impl actix_web::Responder {

// }
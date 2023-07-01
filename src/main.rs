mod routes;

fn build_config() -> String {
    format!(
        "host={} port={} user={} password={} dbname={}",
        std::env::var("DB_HOST").unwrap(),
        std::env::var("DB_PORT").unwrap(),
        std::env::var("DB_USER").unwrap(),
        std::env::var("DB_PASSWORD").unwrap(),
        std::env::var("DB_NAME").unwrap()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let mut db_type_s = String::new();
    let db_type = std::env::var("DB_TYPE");
    match db_type {
        Ok(s) => db_type_s = s,
        Err(_) => panic!("Database type not found!"),
    }

    let postgres_db: std::sync::Arc<Option<tokio_postgres::Client>>;
    let sqlite3_db: std::sync::Arc<Option<tokio_rusqlite::Connection>>;

    if db_type_s.to_uppercase() == "POSTGRES" {
        match tokio_postgres::connect(build_config().as_str(), tokio_postgres::NoTls).await {
            Ok((client, conn)) => {
                postgres_db = std::sync::Arc::new(Some(client));
                sqlite3_db = std::sync::Arc::new(None);
                tokio::spawn(async move {
                    conn.await.expect("Lost database connection. Restarting.");
                    panic!("Lost database connection. :(");
                });
            }
            Err(e) => {
                panic!("Cannot connect to database! {}", e);
            }
        }
    } else {
        // Assuming it's SQLite3
        let sqlite_path = std::env::var("SQLITE_PATH").expect("SQLite path not found!");
        match tokio_rusqlite::Connection::open(&sqlite_path).await {
            Ok(connection) => {
                postgres_db = std::sync::Arc::new(None);
                sqlite3_db = std::sync::Arc::new(Some(connection));
            }
            Err(e) => {
                panic!("Cannot connect to SQLite database! {}", e);
            }
        }
    }

    actix_web::HttpServer::new(move || {
        actix_web::App::new().service(routes::redirect::add).app_data(
            actix_web::web::Data::new(routes::types::States {
                postgres_db: postgres_db.clone(),
                sqlite3_db: sqlite3_db.clone()
            }),
        )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

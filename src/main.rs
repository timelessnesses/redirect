use env_logger;
use log;
mod routes;

fn build_config() -> String {
    format!(
        "postgresql://{}:{}@{}:{}/{}",
        std::env::var("DB_USER").unwrap(),
        std::env::var("DB_PASSWORD").unwrap(),
        std::env::var("DB_HOST").unwrap(),
        std::env::var("DB_PORT").unwrap(),
        std::env::var("DB_NAME").unwrap()
    )
}

const SQL: &str = include_str!("./data/sqls.sql");

struct CustomTargetAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA{
    file: std::fs::File
}

impl CustomTargetAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA {
    fn new() -> Result<CustomTargetAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA, std::io::Error> {
        let x = std::fs::File::create("log.txt");
        match x {
            Ok(f) => return Ok(CustomTargetAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA {file: f}),
            Err(e) => {
                log::warn!("Cannot create log file.");
                return Err(e);
            }
        }
    }
}

impl std::io::Write for CustomTargetAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let _ = std::io::stdout().write(buf);
        self.file.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match std::io::stdout().flush() {
            Err(e) => return Err(e),
            Ok(_) => {
                match self.file.flush() {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(e)
                }
            }
        }
    }
}

fn config_logger() {
    let mut b = env_logger::Builder::from_default_env();
    b.filter_level(log::LevelFilter::Debug);
    b.target(env_logger::Target::Stdout);
    let x = CustomTargetAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA::new();

    match x {
        Ok(file_info) => {
            b.target(env_logger::Target::Pipe(Box::new(file_info)));
        },
        Err(e) => {
            log::warn!("{}",format!("Logger cannot create or write to log.txt. {}", e));
        }
    }
    b.init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    config_logger();

    let db_type_s: String;
    let db_type = std::env::var("DB_TYPE");
    match db_type {
        Ok(s) => db_type_s = s,
        Err(_) => {
            log::error!("No enviroment variable found for DB_TYPE.");
            panic!()
        },
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
                    log::error!("Lost database connection. :(");
                    panic!();
                });
                log::info!("Successfully connected to PostgreSQL!");
            }
            Err(e) => {
                log::error!("Cannot connect to database! {}", e);
                panic!();
            }
        }
    } else if db_type_s.to_uppercase() == "SQLITE3" {
        // Assuming it's SQLite3
        let sqlite_path = std::env::var("SQLITE_PATH").expect("SQLite path not found!");
        match tokio_rusqlite::Connection::open(&sqlite_path).await {
            Ok(connection) => {
                postgres_db = std::sync::Arc::new(None);
                sqlite3_db = std::sync::Arc::new(Some(connection));
                log::info!("Successfully created SQLite3 database file!");
            }
            Err(e) => {
                log::error!("Cannot connect to SQLite database! {}", e);
                panic!();
            }
        }
    } else {
        log::error!("Not valid database choice. You can have either POSTGRES or SQLITE3");
        panic!();
    }

    match (postgres_db.is_some(), sqlite3_db.is_some()) {
        (true, false) => {
            match postgres_db.as_ref().as_ref().unwrap().execute(SQL, &[]).await {
                Err(e) => {
                    log::error!("Cannot create a table! {}",e);
                    panic!();
                }
                _ => {}
            }
        },
        (false, true) => {
            match sqlite3_db.as_ref().as_ref().unwrap().call(
                |c| {
                    c.execute(SQL, [])
                }
            ).await {
                Err(e) => {
                    log::error!("Cannot create a table! {}",e);
                    panic!()
                }
                _ => {}
            }
        },
        _ => {}
    }

    log::info!("Running server! Check it out at http://localhost:8000");

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
        .service(routes::redirect::add)
        .service(routes::redirect::get)
        .app_data(
            actix_web::web::Data::new(routes::types::States {
                postgres_db: postgres_db.clone(),
                sqlite3_db: sqlite3_db.clone()
            }),
        )
        .wrap(actix_web::middleware::Logger::default())
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

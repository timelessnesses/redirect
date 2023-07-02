use serde_derive;
use tokio_postgres;
use tokio_rusqlite;

#[derive(serde_derive::Deserialize)]
pub struct AddParameters {
    pub url: String,
}

pub type UpdateParamaters = AddParameters;

pub struct States {
    pub postgres_db: std::sync::Arc<Option<tokio_postgres::Client>>,
    pub sqlite3_db: std::sync::Arc<Option<tokio_rusqlite::Connection>>,
}

pub struct StatsRequestedData {
    pub url: String,
    pub accessed: i64, // god damn you sqlite3
}

pub struct ListingRequestedData {
    pub url: String,
    pub accessed: i64,
    pub id: String,
}

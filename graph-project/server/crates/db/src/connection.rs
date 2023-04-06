use sqlx::{Error, Pool, Postgres};

pub async fn connect(max_connections: u32) -> Result<Pool<Postgres>, Error> {
    let username = std::env::var("POSTGRES_USER").expect("variable POSTGRES_USER was missing");
    let password =
        std::env::var("POSTGRES_PASSWORD").expect("variable POSTGRES_PASSWORD was missing");
    let host = std::env::var("POSTGRES_HOST").expect("variable POSTGRES_HOST was missing");
    let database =
        std::env::var("POSTGRES_DATABASE").expect("variable POSTGRES_DATABASE was missing");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&format!(
            "postgres://{}:{}@{}/{}",
            username, password, host, database
        ))
        .await
}

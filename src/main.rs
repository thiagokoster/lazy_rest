use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Pool, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://db/lazy_rest.db";

#[derive(Serialize, Deserialize, Debug)]
struct Storage {
    requests: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    name: String,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = create_db().await?;
    println!("Done.");

    println!("Adding a request");

    get_requests(&db).await;

    Ok(())
}

async fn create_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}...", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create database success"),
            Err(error) => panic!("error while creating database: {}", error),
        }
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS request (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
    )",
    )
    .execute(&db)
    .await
    .unwrap();

    Ok(db)
}

async fn get_requests(db: &Pool<Sqlite>) {
    let result = sqlx::query("SELECT name FROM request")
        .fetch_all(db)
        .await
        .unwrap();

    for row in result.iter() {
        println!("Found request {:?}", row.get::<String, &str>("name"));
    }
}

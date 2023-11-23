use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Row, Sqlite, SqlitePool};
use std::env;

#[derive(Clone, FromRow, Debug)]
struct Request {
    name: String,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = create_db().await?;
    println!("Done.");

    println!("Adding a request");
    let request = Request {
        name: "Hello there".to_string(),
    };
    let result = add_request(&db, request).await?;
    if !result {
        panic!("error");
    }

    get_requests(&db).await;

    Ok(())
}

async fn create_db() -> anyhow::Result<Pool<Sqlite>> {
    let database_url = env::var("DATABASE_URL")?;
    println!("Connecting to database: {}", database_url);
    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        println!("Creating database {}...", database_url);
        match Sqlite::create_database(&database_url).await {
            Ok(_) => println!("Create database success"),
            Err(error) => panic!("error while creating database: {}", error),
        }
    }

    let db = SqlitePool::connect(&database_url).await.unwrap();
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
    let result: Vec<Request> = sqlx::query_as("SELECT name FROM request")
        .fetch_all(db)
        .await
        .unwrap();

    for row in result.iter() {
        println!("Found request {:?}", row.name);
    }
}

async fn add_request(db: &Pool<Sqlite>, request: Request) -> anyhow::Result<bool> {
    let result = sqlx::query!(
        r#"
        INSERT INTO request (name) 
        VALUES (?)
    "#,
        request.name
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(result > 0)
}

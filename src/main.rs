use clap::{Parser, Subcommand};
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use std::env;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        method: String,
        url: String,
        name: String,
    },
}

#[derive(Clone, FromRow, Debug)]
struct Request {
    name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { method, url, name } => {
            println!("{}: {} {}", name, method, url);
        }
    }
    return Ok(());

    let pool = create_db().await?;
    println!("Done.");

    println!("Adding a request");
    let request = Request {
        name: "Hello there".to_string(),
    };
    let result = add_request(&pool, request).await?;
    if !result {
        panic!("error");
    }

    get_requests(&pool).await;

    Ok(())
}

async fn create_db() -> anyhow::Result<SqlitePool> {
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

    let pool = SqlitePool::connect(&database_url).await?;
    _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS request (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
    )",
    )
    .execute(&pool)
    .await
    .unwrap();

    Ok(pool)
}

async fn get_requests(pool: &SqlitePool) {
    let result: Vec<Request> = sqlx::query_as("SELECT name FROM request")
        .fetch_all(pool)
        .await
        .unwrap();

    for row in result.iter() {
        println!("Found request {:?}", row.name);
    }
}

async fn add_request(pool: &SqlitePool, request: Request) -> anyhow::Result<bool> {
    let result = sqlx::query!(
        r#"
        INSERT INTO request (name) 
        VALUES (?)
    "#,
        request.name
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(result > 0)
}

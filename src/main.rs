use clap::{Parser, Subcommand};
use sqlx::{FromRow, SqlitePool};
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
        #[arg(short, long)]
        name: String,
    },
    List,
    Delete {
        id: u32,
    },
}

#[derive(Clone, FromRow, Debug)]
struct Request {
    id: Option<u32>,
    name: String,
    method: String,
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    println!("Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(&database_url).await?;

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { method, url, name } => {
            println!("Adding a request");
            println!("{}: {} {}", name, method, url);
            let request = Request {
                id: None,
                name: name.to_string(),
                method: method.to_string(),
                url: url.to_string(),
            };
            let result = add_request(&pool, request).await?;
            if !result {
                panic!("error");
            }
        }
        Commands::List => {
            let _ = get_requests(&pool).await;
        }
        Commands::Delete { id } => {
            println!("Deleting request with id: {}", id);
            let result = delete_request(&pool, id).await?;
            if !result {
                panic!("Error while deleting request with id: {}", id);
            }
        }
    };
    return Ok(());
}

async fn get_requests(pool: &SqlitePool) -> anyhow::Result<bool> {
    let result: Vec<Request> = sqlx::query_as("SELECT id, name, method, url FROM request")
        .fetch_all(pool)
        .await
        .unwrap();

    println!("ID NAME   METHOD  URL");
    for row in result.iter() {
        println!(
            "{} {}    {}  {}",
            row.id.unwrap(),
            row.name,
            row.method,
            row.url
        );
    }
    Ok(true)
}

async fn add_request(pool: &SqlitePool, request: Request) -> anyhow::Result<bool> {
    let result = sqlx::query!(
        r#"
        INSERT INTO request (name, method, url) 
        VALUES (?, ?, ?)
    "#,
        request.name,
        request.method,
        request.url
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(result > 0)
}

async fn delete_request(pool: &SqlitePool, id: &u32) -> anyhow::Result<bool> {
    let result = sqlx::query!(
        r#"
    DELETE FROM request
    WHERE id = ?
    "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();
    Ok(result > 0)
}

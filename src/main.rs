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
        method: Method,
        url: String,
        #[arg(short, long)]
        name: String,
    },
    List,
    Delete {
        id: i64,
    },
    Execute {
        id: i64,
    },
}

#[derive(clap::ValueEnum, sqlx::Type, Clone, Debug)]
enum Method {
    #[sqlx(rename = "get")]
    GET,
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "get" => Method::GET,
            _ => panic!("invalid method"),
        }
    }
}

#[derive(Clone, FromRow, Debug)]
struct Request {
    id: Option<i64>,
    name: String,
    method: Method,
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    println!("Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(&database_url).await?;
    let client = reqwest::Client::new();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { method, url, name } => {
            println!("Adding a request");
            let request = Request {
                id: None,
                name: name.to_string(),
                method: method.clone(),
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
            println!("Deleting request: {}", id);
            let result = delete_request(&pool, id).await?;
            if !result {
                panic!("Error while deleting request: {}", id);
            }
        }
        Commands::Execute { id } => {
            println!("Executing request: {}", id);
            let response = execute_request(&pool, &client, id).await?;
            println!("{}", response);
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
            "{} {}    {:?}  {}",
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

async fn delete_request(pool: &SqlitePool, id: &i64) -> anyhow::Result<bool> {
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

async fn execute_request(
    pool: &SqlitePool,
    client: &reqwest::Client,
    id: &i64,
) -> anyhow::Result<String> {
    let request: Request = sqlx::query_as!(
        Request,
        r#"SELECT id, name, method, url
    FROM request
    WHERE id = ?"#,
        id
    )
    .fetch_one(pool)
    .await?;

    println!(
        "Executing request {}: {:?} {}",
        request.name, request.method, request.url
    );

    let response = match request.method {
        Method::GET => client.get(&request.url).send().await?.text().await?,
    };

    Ok(response)
}

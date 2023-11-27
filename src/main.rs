use clap::Parser;
use cli::Commands;
use request_service::RequestService;
use sqlx::SqlitePool;
use std::env;

mod cli;
mod models;
mod request_service;

struct App<'a> {
    request_service: &'a RequestService<'a>,
}

impl<'a> App<'a> {
    fn new(request_service: &'a RequestService) -> Self {
        App { request_service }
    }

    async fn run(&self) -> anyhow::Result<()> {
        let commands = cli::Cli::parse();
        match commands.command {
            Commands::Add { method, url, name } => {
                println!("Adding a request");
                let request = models::Request {
                    id: None,
                    name: name.to_string(),
                    method: method.clone(),
                    url: url.to_string(),
                };
                let result = self.request_service.add_request(request).await?;
                if !result {
                    panic!("error");
                }
            }
            Commands::List => {
                let _ = self.request_service.get_requests().await;
            }
            Commands::Delete { id } => {
                println!("Deleting request: {}", id);
                let result = self.request_service.delete_request(&id).await?;
                if !result {
                    panic!("Error while deleting request: {}", id);
                }
            }
            Commands::Execute { id } => {
                println!("Executing request: {}", id);
                let response = self.request_service.execute_request(&id).await?;
                println!("{}", response);
            }
        };

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    println!("Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(&database_url).await?;
    let client = reqwest::Client::new();
    let request_service = request_service::RequestService::new(&pool, &client);

    let app = App::new(&request_service);

    app.run().await?;

    Ok(())
}

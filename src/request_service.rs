use std::collections::HashMap;
use std::io::Write;

use std::fmt::{self, Display};

use serde_json::json;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::models;
use sqlx::SqlitePool;

pub struct RequestService<'a> {
    pool: &'a SqlitePool,
    client: &'a reqwest::Client,
}

impl<'a> RequestService<'a> {
    pub fn new(pool: &'a SqlitePool, client: &'a reqwest::Client) -> Self {
        RequestService { pool, client }
    }

    pub async fn get_requests(&self) -> anyhow::Result<bool> {
        let result: Vec<models::Request> =
            sqlx::query_as("SELECT id, name, method, url FROM request")
                .fetch_all(self.pool)
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

    pub async fn add_request(&self, request: models::Request) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"
        INSERT INTO request (name, method, url) 
        VALUES (?, ?, ?)
    "#,
            request.name,
            request.method,
            request.url
        )
        .execute(self.pool)
        .await?
        .rows_affected();

        Ok(result > 0)
    }

    pub async fn delete_request(&self, id: &i64) -> anyhow::Result<bool> {
        let result = sqlx::query!(
            r#"
    DELETE FROM request
    WHERE id = ?
    "#,
            id
        )
        .execute(self.pool)
        .await?
        .rows_affected();
        Ok(result > 0)
    }

    pub async fn execute_request(&self, id: &i64) -> anyhow::Result<Response> {
        let request = sqlx::query_as!(
            models::Request,
            r#"SELECT id, name, method, url
    FROM request
    WHERE id = ?"#,
            id
        )
        .fetch_one(self.pool)
        .await?;

        println!(
            "Executing request {}: {:?} {}",
            request.name, request.method, request.url
        );

        let response = match request.method {
            models::Method::GET => self.client.get(&request.url).send().await?,
        };

        let response = Response {
            headers: response
                .headers()
                .iter()
                .map(|(key, value)| {
                    (
                        key.as_str().to_string(),
                        value.to_str().unwrap().to_string(),
                    )
                })
                .collect(),
            body: response.text().await?,
        };

        Ok(response)
    }
}

pub struct Response {
    headers: HashMap<String, String>,
    body: String,
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = BufferWriter::stdout(ColorChoice::Always).buffer();
        buffer.set_color(ColorSpec::new().set_bold(true)).unwrap();

        _ = writeln!(buffer, "Headers");
        _ = buffer.reset();
        for (key, value) in self.headers.iter() {
            _ = writeln!(buffer, "  {}: {}", key, value);
        }

        buffer.set_color(ColorSpec::new().set_bold(true)).unwrap();
        _ = writeln!(buffer, "Body");
        _ = buffer.reset();
        _ = writeln!(buffer, "  {}", self.body);

        write!(f, "{}", String::from_utf8_lossy(&buffer.into_inner()))
    }
}

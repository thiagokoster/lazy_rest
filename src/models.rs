#[derive(clap::ValueEnum, sqlx::Type, Clone, Debug)]
pub enum Method {
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

#[derive(Clone, sqlx::FromRow, Debug)]
pub struct Request {
    pub id: Option<i64>,
    pub name: String,
    pub method: Method,
    pub url: String,
}

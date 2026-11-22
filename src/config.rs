use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        Self { host, port }
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

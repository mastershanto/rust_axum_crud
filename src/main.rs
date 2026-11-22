use axum::{routing::get, Router};

mod config;
mod error;
mod models;
mod routes;
mod states;

use config::Config;
use states::AppState;

#[tokio::main]
async fn main() {
    // 1. Initialize logging
    tracing_subscriber::fmt::init();

    // 2. Load configuration from environment / defaults
    let config = Config::init();

    // 3. Initialize application state
    let state = AppState::new();

    // 4. Build the router with state
    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/users", routes::user_routes::user_routes())
        .with_state(state);

    // 5. Bind to address and start listening
    let addr = config.server_addr();
    tracing::info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind TCP Listener on {}", addr));

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn root_handler() -> &'static str {
    "Hello from axum"
}
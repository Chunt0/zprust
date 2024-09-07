use sqlx::PgPool;
use std::net::TcpListener;
use zprust::config::get_config;
use zprust::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to read config.yaml");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres...");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to address...");
    run(listener, connection_pool).expect("Failed to run").await
}

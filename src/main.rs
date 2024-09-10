use core::panic;
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zprust::config::get_config;
use zprust::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let config = match get_config() {
        Ok(config) => config,
        Err(e) => panic!("Failed to read config.yaml... {e}"),
    };

    let connection_pool = match PgPool::connect(&config.database.connection_string()).await {
        Ok(connection_pool) => connection_pool,
        Err(e) => panic!("Failed to connect to Postgres... {e}"),
    };

    let address = format!("127.0.0.1:{}", config.application_port);

    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(e) => panic!("Failed to bind to address... {e}"),
    };

    run(listener, connection_pool).expect("Failed to run").await
}

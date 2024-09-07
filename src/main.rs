use std::net::TcpListener;
use zprust::config::get_config;
use zprust::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to read configuration");
    println!("{:#?}", config);
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to address...");
    run(listener)?.await
}

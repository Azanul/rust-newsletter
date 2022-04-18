use env_logger::Env;
use rust_newsletter::configuration::get_configuration;
use rust_newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration.");

    // We have removed the hard-coded `8000` - it's now coming from our settings!

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}

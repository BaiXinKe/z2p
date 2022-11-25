use std::{error::Error, net::TcpListener};

use secrecy::ExposeSecret;
use sqlx::PgPool;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection =
        PgPool::connect(configuration.database.connection_string().expose_secret()).await?;

    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let listen: TcpListener = TcpListener::bind(addr)?;

    run(listen, connection)?.await?;

    Ok(())
}

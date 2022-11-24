use std::{error::Error, net::TcpListener};

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string()).await?;

    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let listen: TcpListener = TcpListener::bind(addr)?;

    run(listen, connection)?.await?;

    Ok(())
}

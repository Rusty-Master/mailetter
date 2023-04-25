use std::net::TcpListener;

use mailetter::{configuration::get_configuration, startup::run};
use sqlx::{Connection, PgConnection, PgPool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

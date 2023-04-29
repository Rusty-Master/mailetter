use std::net::TcpListener;

use mailetter::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("mailetter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}

use std::net::TcpListener;

use curium_services::{
    config::{
        logging::{create_logging_subscriber, init_subscriber},
        settings::read_configuration,
    },
    server::run,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = create_logging_subscriber("curium-service".into(), "info");
    init_subscriber(subscriber);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let settings = read_configuration().expect("Failed to read configuration");
    let pg_pool = PgPoolOptions::new()
        .min_connections(2)
        .connect_lazy_with(settings.db.get_options());
    let listener =
        TcpListener::bind(settings.application.get_addr()).expect("Failed to bind address");

    let server = run(listener, settings, pg_pool).await;
    server?.await
}

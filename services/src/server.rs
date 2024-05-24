use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http::header,
    middleware::Logger,
    web::{self, scope, ServiceConfig},
    App, HttpServer,
};
use sqlx::{Pool, Postgres};

use crate::{
    api::actuator::health_check_handler,
    config::settings::{AppSettings, Settings},
};

#[doc = "Application State"]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub settings: Settings,
}

#[doc = "Setup the service served by the application"]
fn get_config(conf: &mut ServiceConfig, settings: &AppSettings) {
    conf.service(scope(&settings.api_prefix).service(health_check_handler));
}

#[doc = "Configure CORS"]
fn cors(settings: &AppSettings) -> Cors {
    if &settings.cors_location != "*" {
        Cors::default()
            .allowed_origin(&settings.cors_location)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::AUTHORIZATION,
            ])
            .supports_credentials()
    } else {
        Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::AUTHORIZATION,
            ])
            .supports_credentials()
    }
}

#[doc = "runnable server"]
pub async fn run(
    tcp_listener: TcpListener,
    settings: Settings,
    pg_pool: Pool<Postgres>,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pg_pool.clone(),
                settings: settings.clone(),
            }))
            .configure(|c| get_config(c, &settings.application))
            .wrap(cors(&settings.application))
            .wrap(Logger::default())
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}

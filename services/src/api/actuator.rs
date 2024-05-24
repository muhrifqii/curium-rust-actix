use actix_web::{get, HttpResponse};

#[tracing::instrument(name = "Checking the application health")]
#[get("/actuator/health_check")]
pub async fn health_check_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde_json::json;

use crate::{
    domain::user::UserResponse,
    model::{request::registration::RegistrationRequest, response::ErrorResponse},
    server::AppState,
    service::registration::register_new_user,
};

#[doc = r#"API Resource: /auth/register [POST]

Register a new user to the application providing the required details.

If the operation is successfull, 200 OK is returned with the created user's details.

Errors:

ErrorCode::REG001 / 409 Conflict - The username already exists.
ErrorCode::REG002 / 409 Conflict - The email already exists.
ErrorCode::Unhandled / 500 Internal Server Error - Any other error case.
"#]
#[tracing::instrument(
    name = "Registering a new user",
    skip(body, data),
    fields(
        username = body.username,
        email = body.email
    )
)]
#[post("/auth/register")]
pub async fn register_user_handler(
    body: Json<RegistrationRequest>,
    data: Data<AppState>,
) -> HttpResponse {
    let response: Result<UserResponse, ErrorResponse> =
        register_new_user(&body.into_inner(), &data.db).await;

    match response {
        Ok(user) => HttpResponse::Ok().json(json!(user)),
        Err(c) => c.build(),
    }
}

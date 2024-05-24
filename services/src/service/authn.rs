use chrono::Utc;
use sqlx::PgPool;

use crate::{
    config::settings::Settings,
    domain::user::AuthUser,
    model::{authn::TokenClaims, request::auth::LoginRequest, response::ErrorResponse},
};

pub async fn login(
    data: &LoginRequest,
    settings: &Settings,
    pool: &PgPool,
) -> Result<String, ErrorResponse> {
    let user = AuthUser::find_by_identifier(&data.identifier, pool)
        .await?
        .ok_or(ErrorResponse::Authn001)?;

    let now = Utc::now();
    let iat: usize = now.timestamp() as usize;
    let exp: usize = (now + chrono::Duration::minutes(settings.jwt.max_age)).timestamp() as usize;
    let role: String = user.role.to_owned();
    let claims: TokenClaims = TokenClaims {
        sub: user.user_id.to_string(),
        exp,
        iat,
        role,
    };

    Ok("".to_string())
}

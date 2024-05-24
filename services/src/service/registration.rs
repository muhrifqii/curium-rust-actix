use argon2::password_hash::{rand_core, SaltString};
use argon2::{Argon2, PasswordHasher};
use rand_core::OsRng;
use sqlx::{query_as, PgPool};

use crate::domain::user::UserResponse;
use crate::model::request::registration::RegistrationRequest;
use crate::model::response::ErrorResponse;

#[doc = r#"
Register a user to the application.

After the user is created, the details are returned (excluding the password).
"#]
pub async fn register_new_user(
    data: &RegistrationRequest,
    pool: &PgPool,
) -> Result<UserResponse, ErrorResponse> {
    // Check if the provided username is already in use.
    if query_as(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM app_user u
            WHERE LOWER(u.username) = LOWER($1)
        )
        "#,
    )
    .bind(&data.username)
    .fetch_one(pool)
    .await
    .map(|r: (bool,)| r.0)
    .map_err(|e| ErrorResponse::Unhandled(Some(e.to_string())))?
    {
        return Err(ErrorResponse::Reg001);
    }

    // Check if the provided email is in use
    if query_as(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM app_user u
            WHERE LOWER(u.email) = LOWER($1)
        )
        "#,
    )
    .bind(&data.email)
    .fetch_one(pool)
    .await
    .map(|r: (bool,)| r.0)
    .map_err(|e| ErrorResponse::Unhandled(Some(e.to_string())))?
    {
        return Err(ErrorResponse::Reg002);
    }

    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .map_err(|e| ErrorResponse::Unhandled(Some(e.to_string())))?
        .to_string();

    query_as(
        r#"
        INSERT INTO app_user (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING user_id, username, email, role, created_at, updated_at
        "#,
    )
    .bind(&data.username)
    .bind(&data.email)
    .bind(hashed_password)
    .fetch_one(pool)
    .await
    .map_err(|e| ErrorResponse::Unhandled(Some(e.to_string())))
}

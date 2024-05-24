use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

use crate::model::response::ErrorResponse;

#[doc = "User Model used for authn"]
#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AuthUser {
    #[doc = r#"Retrieve a user from @app_user by the provided identifier (email or username).
    In any error ErrorCode::INTERNAL001 is returned"#]
    pub async fn find_by_identifier(
        identifier: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, ErrorResponse> {
        query_as(
            r#"
            SELECT u.user_id,
                   u.username,
                   u.email,
                   u.password,
                   u.role
            FROM app_user u
            WHERE u.username ILIKE $1
               OR u.email ILIKE $1
            "#,
        )
        .bind(identifier)
        .fetch_optional(pool)
        .await
        .map_err(|e| ErrorResponse::Unhandled(Some(e.to_string())))
    }

    #[doc = "Update the user's last login with the current timestamp."]
    pub async fn db_update_last_login(&self, pool: &PgPool) -> Result<(), ErrorResponse> {
        let query_result = query(
            r#"
            UPDATE app_user
            SET last_login = now()
            WHERE user_id = $1
            "#,
        )
        .bind(self.user_id)
        .execute(pool)
        .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorResponse::Unhandled(Some(e.to_string()))),
        }
    }
}

impl UserResponse {
    pub async fn find_by_id(user_id: i64, pool: &PgPool) -> Result<Option<Self>, ErrorResponse> {
        query_as(
            r#"
            SELECT u.user_id,
                   u.username,
                   u.email,
                   u.role,
                   u.created_at,
                   u.updated_at
            FROM app_user u
            WHERE u.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ErrorResponse::Unhandled(Some(e.to_string())))
    }
}

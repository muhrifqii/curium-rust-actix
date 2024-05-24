use core::fmt;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    code: u32,
    message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorResponse {
    #[doc = "User/Password invalid."]
    Authn001,

    #[doc = "Username already exists."]
    Reg001,
    #[doc = "Email already exists."]
    Reg002,

    #[doc = "Too many attempts have been made."]
    RateLimit(Option<String>),
    #[doc = "Generic Internal Error."]
    Unhandled(Option<String>),
}

impl ErrorResponse {
    pub fn build(&self) -> HttpResponse {
        let body = self.into_response_body();
        match self {
            ErrorResponse::Authn001 => HttpResponse::NotFound(),
            ErrorResponse::Reg001 | ErrorResponse::Reg002 => HttpResponse::Conflict(),
            ErrorResponse::RateLimit(_) => HttpResponse::TooManyRequests(),
            ErrorResponse::Unhandled(_) => HttpResponse::InternalServerError(),
        }
        .json(json!(body))
    }

    pub fn into_response_body(&self) -> ErrorResponseBody {
        let (code, default_message) = match self {
            ErrorResponse::Authn001 => (1001, "User/Password invalid."),
            ErrorResponse::Reg001 => (1101, "Username already exists."),
            ErrorResponse::Reg002 => (1102, "Email already exists."),
            ErrorResponse::RateLimit(_) => (8110, "Too many attempts have been made."),
            ErrorResponse::Unhandled(_) => (9999, "Generic Internal Error."),
        };

        let message = match self {
            ErrorResponse::Authn001 | ErrorResponse::Reg001 | ErrorResponse::Reg002 => None,

            ErrorResponse::RateLimit(msg) | ErrorResponse::Unhandled(msg) => {
                msg.clone().or(Some(default_message.to_string()))
            }
        };
        ErrorResponseBody { code, message }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self.into_response_body()).unwrap()
        )
    }
}

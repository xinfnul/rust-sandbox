use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::{Value, json};
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid request body")]
    BadRequest,

    #[error("request validation failed")]
    Validation(#[from] ValidationErrors),

    #[error("todo not found")]
    NotFound,

    #[error("unable to allocate a todo ID")]
    IdExhausted,

    #[error("Internal server error")]
    Internal(#[source] anyhow::Error),
}

impl AppError {
    pub fn internal(error: impl Into<anyhow::Error>) -> Self {
        Self::Internal(error.into())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::IdExhausted | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Self::BadRequest => "bad_request",
            Self::Validation(_) => "validation_error",
            Self::NotFound => "not_found",
            Self::IdExhausted | Self::Internal(_) => "internal_error",
        }
    }

    fn public_message(&self) -> &'static str {
        match self {
            Self::BadRequest => "Invalid request body",
            Self::Validation(_) => "Request validation failed",
            Self::NotFound => "Todo not found",
            Self::IdExhausted | Self::Internal(_) => "An internal server error occurred",
        }
    }
}

#[derive(Serialize)]
struct ApiError {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let code = self.error_code();
        let message = self.public_message();

        let details = match &self {
            AppError::Validation(errors) => {
                let fields = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| {
                        let messages: Vec<&str> = errors
                            .iter()
                            .map(|error| error.message.as_deref().unwrap_or("Invalid value"))
                            .collect();

                        (field.to_string(), messages)
                    })
                    .collect::<std::collections::HashMap<_, _>>();

                Some(json!(fields))
            }
            _ => None,
        };

        if let AppError::Internal(error) = &self {
            tracing::error!(error = ?error, "Internal application error");
        }

        let body = ApiError {
            error: ErrorDetail {
                code,
                message,
                details,
            },
        };

        (status, Json(body)).into_response()
    }
}

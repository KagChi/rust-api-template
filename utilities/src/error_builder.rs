use axum::{
    http::{StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::ToSchema;

// ---------------------------
// RFC 9457 Error Structures
// ---------------------------

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse<T = ()> {
    #[serde(default = "default_type")]
    pub r#type: String,
    pub title: String,
    pub detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<T>,
}

fn default_type() -> String {
    "about:blank".to_string()
}

#[derive(Debug)]
pub struct ErrorBuilder<T = ()> {
    inner: ErrorResponse<T>,
    status: StatusCode,
}

impl<T: Serialize + Clone + Debug> ErrorBuilder<T> {
    pub fn new(title: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            inner: ErrorResponse {
                r#type: default_type(),
                title: title.into(),
                detail: detail.into(),
                instance: None,
                kind: None,
            },
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn kind(mut self, kind: T) -> Self {
        self.inner.kind = Some(kind);
        self
    }

    pub fn instance(mut self, instance: impl Into<String>) -> Self {
        self.inner.instance = Some(instance.into());
        self
    }

    pub fn r#type(mut self, uri: impl Into<String>) -> Self {
        self.inner.r#type = uri.into();
        self
    }

    pub fn mdn_code(mut self, code: StatusCode) -> Self {
        self.inner.r#type = format!(
            "https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/{}",
            code.as_u16()
        );
        self.status = code;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.inner.title = title.into();
        self
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.inner.detail = detail.into();
        self
    }

    pub fn build(self) -> ErrorResponse<T> {
        self.inner
    }

    pub fn into_response(self) -> Response {
        (self.status, axum::Json(self.inner)).into_response()
    }
}

// Implement IntoResponse for Axum
impl<T: Serialize + Debug> IntoResponse for ErrorBuilder<T> {
    fn into_response(self) -> Response {
        (self.status, axum::Json(self.inner)).into_response()
    }
}
use axum::{
    extract::{FromRequest, Json, Request, rejection::JsonRejection},
    http::StatusCode,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::fmt::Debug;
use validator::{Validate, ValidationErrors};

use crate::error_builder::ErrorBuilder;

#[derive(Debug, Clone, Default)]
pub struct ValidatedJson<T>(pub T);

fn extract_missing_field(err: &JsonRejection) -> Option<String> {
    let msg = err.to_string();

    if let Some(pos) = msg.find("missing field `") {
        msg[pos..].split('`').nth(1).map(|s| s.to_string())
    } else {
        None
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = ErrorBuilder<Value>;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let path = req.uri().clone().path().to_string();

        let Json(value) = Json::<T>::from_request(req, _state).await.map_err(|err| {
            let detail = if let Some(field) = extract_missing_field(&err) {
                format!("Missing required field: `{}`", field)
            } else {
                err.to_string()
            };

            let kind = json!({ "message": detail.clone() });

            ErrorBuilder::new("Invalid Request Body", detail)
                .kind(kind)
                .instance(path.clone())
                .mdn_code(StatusCode::BAD_REQUEST)
        })?;

        value.validate().map_err(|err: ValidationErrors| {
            let binding = err.field_errors();
            let first_field_error = binding.iter().next();

            let (detail, kind_value) = match first_field_error {
                Some((field, errs)) => {
                    if let Some(first_error) = errs.first() {
                        let display_field = if field == "__all__" {
                            first_error
                                .params
                                .get("field")
                                .and_then(|v| {
                                    if let serde_json::Value::String(s) = v {
                                        Some(s.as_str())
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(field)
                        } else {
                            field
                        };

                        let message = first_error
                            .message
                            .clone()
                            .unwrap_or_else(|| "".into())
                            .to_string();

                        let detail_msg = format!(
                            "Field '{}' validation failed. Reason: {}.",
                            display_field, message
                        );

                        let mut kind_obj = serde_json::Map::new();
                        kind_obj.insert(
                            "code".into(),
                            serde_json::Value::String(first_error.code.clone().into()),
                        );
                        kind_obj.insert(
                            "field".into(),
                            serde_json::Value::String(display_field.into()),
                        );
                        kind_obj.insert("message".into(), serde_json::Value::String(message));

                        (detail_msg, serde_json::Value::Object(kind_obj))
                    } else {
                        (
                            format!("Validation failed for field '{}'", field),
                            json!({}),
                        )
                    }
                }
                None => (format!("Validation failed: {:?}", err), json!({})),
            };

            ErrorBuilder::new("Validation Error", detail)
                .kind(kind_value)
                .instance(path.clone())
                .mdn_code(StatusCode::UNPROCESSABLE_ENTITY)
        })?;

        Ok(ValidatedJson(value))
    }
}

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use schema::request::user::UserRegistration;
use serde::Serialize;
use utoipa::ToSchema;
use utilities::{error_builder::{ErrorResponse}, validated_json::ValidatedJson};

#[derive(Serialize, ToSchema)]
pub struct MyResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "user",
    request_body = UserRegistration,
    responses(
        (status = 200, description = "User registered successfully", body = MyResponse),
        (status = 400, description = "Invalid input", body = ErrorResponse<String>)
    )
)]
pub async fn route(ValidatedJson(body): ValidatedJson<UserRegistration>) -> Response {
    let msg = format!("Hello, {}!", body.username);
    (StatusCode::OK, Json(MyResponse { message: msg }))
        .into_response()
}

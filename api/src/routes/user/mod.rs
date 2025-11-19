use utoipa_axum::{router::OpenApiRouter, routes};

pub mod register;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register::route))
}
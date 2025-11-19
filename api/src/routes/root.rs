use axum::http::StatusCode;
use axum::Json;
use schema::response::BaseResponse;

pub async fn route() -> (StatusCode, Json<BaseResponse>) {
    (StatusCode::OK, Json(BaseResponse{message: format!("NusaBelajar Auth Service v{}", env!("CARGO_PKG_VERSION"))}))
}
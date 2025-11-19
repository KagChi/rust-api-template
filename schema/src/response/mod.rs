use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BaseResponse {
    pub message: String,
}
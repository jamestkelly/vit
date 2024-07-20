use serde::Serialize;

#[derive(Serialize)]
pub struct BaseResponse {
    pub status: String,
    pub message: String,
}


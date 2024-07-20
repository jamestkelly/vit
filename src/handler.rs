use crate::{ response::BaseResponse, WebResult };
use warp::{ http::StatusCode, reply::json, reply::with_status, Reply };

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Vit API is live.";

    let response_json = &(BaseResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    });
    Ok(json(response_json))
}

use serde::Serialize;

#[derive(Serialize)]
pub struct SimpleResponse {
    pub error: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct CompleteResponse {
    pub error: bool,
    pub message: String,
    pub token: String,
}
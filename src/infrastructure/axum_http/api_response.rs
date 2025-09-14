use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct EmptyResponseModel;
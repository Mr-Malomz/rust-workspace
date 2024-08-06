use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseData {
    pub records: Vec<Project>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectRequest {
    pub name: String,
    pub description: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectResponse {
    pub id: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct APIResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone)]
pub struct APIErrorResponse {
    pub status: u16,
    pub message: String,
    pub data: Option<String>,
}

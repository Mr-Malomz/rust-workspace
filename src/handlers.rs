use actix_web::{http::StatusCode, post, web::Json, HttpResponse};
use shared_models::{APIErrorResponse, APIResponse, Project, ProjectRequest, ProjectResponse};
use xata_client::XataClient;

#[post("/project")]
pub async fn create_project_handler(new_project: Json<ProjectRequest>) -> HttpResponse {
    let create_project = XataClient::create_project(new_project.to_owned()).await;

    match create_project {
        Ok(data) => HttpResponse::Created().json(APIResponse::<ProjectResponse> {
            status: StatusCode::CREATED.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

#[post("/projects")]
pub async fn get_projects_handler() -> HttpResponse {
    let get_projects = XataClient::get_projects().await;

    match get_projects {
        Ok(data) => HttpResponse::Ok().json(APIResponse::<Vec<Project>> {
            status: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(data),
        }),
        Err(error) => HttpResponse::InternalServerError().json(APIErrorResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: Some(error.to_string()),
        }),
    }
}

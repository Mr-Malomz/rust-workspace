use std::env;

use dotenv::dotenv;
use reqwest::{header, Client, Error};
use shared_models::{Project, ProjectRequest, ProjectResponse, ResponseData};

pub struct XataClient;

impl XataClient {
    fn env_loader(key: &str) -> String {
        dotenv().ok();
        match env::var(key) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading environment variable"),
        }
    }

    fn init() -> Client {
        Client::new()
    }

    fn create_header() -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", XataClient::env_loader("XATA_API_KEY"))
                .parse()
                .unwrap(),
        );

        headers
    }

    pub async fn create_project(new_project: ProjectRequest) -> Result<ProjectResponse, Error> {
        let database_url = XataClient::env_loader("XATA_DATABASE_URL");
        let url = format!("{}:main/tables/Project/data", database_url);

        let client = XataClient::init()
            .post(url)
            .headers(XataClient::create_header())
            .json(&new_project)
            .send()
            .await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                let created_project: ProjectResponse = serde_json::from_str(json.as_str()).unwrap();
                Ok(created_project)
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_projects() -> Result<Vec<Project>, Error> {
        let database_url = XataClient::env_loader("XATA_DATABASE_URL");
        let url = format!("{}:main/tables/Project/query", database_url);

        let client = XataClient::init()
            .post(url)
            .headers(XataClient::create_header())
            .send()
            .await;

        match client {
            Ok(response) => {
                let json = response.text().await?;
                let response_data: ResponseData = serde_json::from_str(json.as_str()).unwrap();
                Ok(response_data.records)
            }
            Err(error) => Err(error),
        }
    }
}

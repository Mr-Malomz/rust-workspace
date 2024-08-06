use actix_web::{App, HttpServer};
use handlers::{create_project_handler, get_projects_handler};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(create_project_handler)
            .service(get_projects_handler)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

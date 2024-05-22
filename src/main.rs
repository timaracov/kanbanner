use actix_web::{HttpServer, App};
mod api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::handlers::get_all_projects)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

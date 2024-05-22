use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


#[get("/projects")]
async fn get_all_projects() -> impl Responder {
    return HttpResponse::Ok().body("ok");
}

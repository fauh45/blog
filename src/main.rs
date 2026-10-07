use std::env;

use actix_web::{App, HttpServer, Responder, get, web};
use personal_web::renderer;

#[get("/")]
async fn main_page() -> impl Responder {
    renderer::handle_page_request("main".to_string())
}

#[get("/{page}")]
async fn other_page(path: web::Path<(String,)>) -> impl Responder {
    renderer::handle_page_request(path.into_inner().0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = env::var("BIND_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .expect("PORT should be parse-able");

    println!("Listening on {}:{}", bind_addr, app_port);

    HttpServer::new(|| App::new().service(main_page).service(other_page))
        .bind((bind_addr, app_port))?
        .run()
        .await
}

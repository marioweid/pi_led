use std::error::Error;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod backend;
#[get("/")]
async fn hello() -> impl Responder {
    let _res: Result<(), Box<dyn Error>> = backend::business_logic();
    print!("calling hello() ...");
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}
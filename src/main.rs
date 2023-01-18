use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


struct  AppState {
  app_name: String,
}


#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", data.app_name))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix-web"),
            }))
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}
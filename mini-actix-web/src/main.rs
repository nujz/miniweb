use actix_web::{App, HttpServer, get};

#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .workers(num_cpus::get())
        .run()
        .await
}

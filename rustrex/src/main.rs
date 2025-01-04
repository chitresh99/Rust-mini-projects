use actix_web::{get,App,HttpServer,Responder};

#[get("/")]
async fn rex() -> impl Responder{
    "Hello , welcome to rex"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(rex)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
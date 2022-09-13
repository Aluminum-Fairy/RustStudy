use actix_web::{get,post,web::{self, Query},App,HttpResponse,HttpServer,Responder};


#[get("/")]
async fn hello()-> impl Responder{
    HttpResponse::Ok().body("Hello World")
}

#[post("/")]
async fn echo(query:String,times:String) -> impl Responder{
    HttpResponse::Ok().body(query+&times)
}

async fn manual_hello() -> impl Responder{
    HttpResponse::Ok().body("Hey There")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey",web::get().to(manual_hello))
    })
    .bind(("0.0.0.0",8081))?
    .run()
    .await
}
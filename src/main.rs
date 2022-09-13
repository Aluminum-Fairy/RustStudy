use actix_web::{web,App,HttpServer,Responder, get};

struct AppState{
    app_name :String,
}

#[get("/")]
async fn index(data: web ::Data<AppState>) -> impl Responder{
    let app_name = &data.app_name;      //get App Name
    format!("Hello {app_name}")                 //response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .app_data(web::Data::new(AppState{
                app_name: String ::from ("Actix Web"),
            }))
            .service(index)
    })
    .bind(("0.0.0.0",8081))?
    .run()
    .await
}
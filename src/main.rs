use actix_web::{web,App,HttpServer,Responder, get};
use std::sync::Mutex;

struct AppState{
    counter :Mutex<i32>,
    app_name :String
}

#[get("/")]
async fn index(data: web ::Data<AppState>) -> String{
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter +=1;
    format!("Hello {app_name} Request Count :{counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .app_data(web::Data::new(AppState{
                app_name: String ::from ("Actix Web"),
                counter: Mutex::new(0)
            }))
            .service(index)
    })
    .bind(("0.0.0.0",8081))?
    .run()
    .await
}
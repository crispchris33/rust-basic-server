use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn trigger_script() -> impl Responder {
    // Here, you would ideally send a command to the client. 
    // For simplicity, we're just returning a response.
    HttpResponse::Ok().body("Script triggered")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/trigger-script", web::get().to(trigger_script))
            // Serve static files (our frontend)
            .service(actix_files::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

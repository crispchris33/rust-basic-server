use actix::{Addr};
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;

mod websocket;
use websocket::MyWebSocket;

struct AppState {
    ws_connections: Arc<Mutex<HashMap<Uuid, Addr<MyWebSocket>>>>,
}

async fn trigger_script(data: web::Data<AppState>) -> impl Responder {
    let connections = data.ws_connections.lock().unwrap();
    for addr in connections.values() {
        addr.do_send(websocket::WebSocketMessage("trigger_script".to_string()));
    }

    HttpResponse::Ok().body("Script trigger request received")
}

async fn ws_index(req: HttpRequest, stream: web::Payload, data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    let id = Uuid::new_v4();
    let my_websocket = MyWebSocket::new(id, data.ws_connections.clone()); // Pass the Arc<Mutex<...>> directly

    let response = ws::start(my_websocket, &req, stream)?;

    Ok(response)
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        ws_connections: Arc::new(Mutex::new(HashMap::new())), // Wrap in Arc
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .route("/trigger-script", web::get().to(trigger_script))
            .route("/ws/", web::get().to(ws_index))
            // ... other routes and services ...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

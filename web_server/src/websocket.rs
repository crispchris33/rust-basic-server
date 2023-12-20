use actix::{Actor,AsyncContext, StreamHandler, Handler, Message};
use actix_web_actors::ws;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Define a type alias for convenience
type WsConnectionMap = Arc<Mutex<HashMap<Uuid, actix::Addr<MyWebSocket>>>>;

// Message structure for sending text to the WebSocket client
#[derive(Message)]
#[rtype(result = "()")]
pub struct WebSocketMessage(pub String);

pub struct MyWebSocket {
    id: Uuid,
    ws_connections: WsConnectionMap,
}

impl MyWebSocket {
    pub fn new(id: Uuid, ws_connections: WsConnectionMap) -> Self {
        MyWebSocket { id, ws_connections }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Register self in the AppState when starting
        let addr = ctx.address();
        self.ws_connections.lock().unwrap().insert(self.id, addr);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        // Deregister self from the AppState when stopping
        self.ws_connections.lock().unwrap().remove(&self.id);
        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, _msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        // Handle incoming WebSocket messages here
    }
}

// Implement handling for `WebSocketMessage`
impl Handler<WebSocketMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WebSocketMessage, ctx: &mut Self::Context) {
        // For example, you can send the message content to the client
        ctx.text(msg.0);
    }
}

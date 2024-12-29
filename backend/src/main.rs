use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Build the application with a route for WebSocket connections
    let app = Router::new().route("/ws", get(ws_handler));

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Bind the TCP listener
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    // Start the server using the new `serve` function
    axum::serve(listener, app).await.expect("Server failed");
}

// Handler to upgrade HTTP requests to WebSocket connections
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

// Function to handle the WebSocket connection
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            let uppercased = text.to_uppercase();
            if socket.send(Message::Text(uppercased)).await.is_err() {
                // Client disconnected
                return;
            }
        }
    }
}

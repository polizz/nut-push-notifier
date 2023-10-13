// https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, TypedHeader,
    },
    headers,
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::StatusEvent;
// use std::ops::ControlFlow;
use std::env;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::info;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

pub async fn startup<'local>(opts: WsServerOpts, rx: tokio::sync::watch::Receiver<StatusEvent>) {
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", get(ws_handler))
        .with_state(rx)
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // run it with hyper
    // let addr = "192.168.1.174:3000";
    let addr = format!("{}:{}", opts.websocket_bind_ip, opts.websocket_bind_port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("ws server listening on {}", listener.local_addr().unwrap());
    axum::Server::from_tcp(listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(rx): State<tokio::sync::watch::Receiver<StatusEvent>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, rx))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    mut rx: tokio::sync::watch::Receiver<StatusEvent>,
) {
    while rx.changed().await.is_ok() {
        let msg_text = { format!("{:?}", rx.borrow_and_update().ups_status.clone()) };
        info!("Sending state to websocket client: {}", who);
        if socket.send(Message::Text(msg_text)).await.is_err() {
            println!("Client {who} abruptly disconnected");
            return;
        }
    }
}

pub struct WsServerOpts {
    pub websocket_bind_ip: String,
    pub websocket_bind_port: String,
}

// helper to print contents of messages to stdout. Has special treatment for Close.
// fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
//     match msg {
//         Message::Text(t) => {
//             println!(">>> {who} sent str: {t:?}");
//         }
//         Message::Binary(d) => {
//             println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
//         }
//         Message::Close(c) => {
//             if let Some(cf) = c {
//                 println!(
//                     ">>> {} sent close with code {} and reason `{}`",
//                     who, cf.code, cf.reason
//                 );
//             } else {
//                 println!(">>> {who} somehow sent close message without CloseFrame");
//             }
//             return ControlFlow::Break(());
//         }
//
//         Message::Pong(v) => {
//             println!(">>> {who} sent pong with {v:?}");
//         }
//         // You should never need to manually handle Message::Ping, as axum's websocket library
//         // will do so for you automagically by replying with Pong and copying the v according to
//         // spec. But if you need the contents of the pings you can see them here.
//         Message::Ping(v) => {
//             println!(">>> {who} sent ping with {v:?}");
//         }
//     }
//     ControlFlow::Continue(())
// }

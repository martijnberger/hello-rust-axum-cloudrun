use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::env;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/hey", get(manual_hello))
        .route("/echo", post(echo_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}

// Just return the request body
async fn echo_handler(body: String) -> impl IntoResponse {
    body.to_owned()
}

async fn manual_hello() -> impl IntoResponse {
    // Test env "TARGET" which defined when `docker run`, or `gcloud run deploy --set-env-vars`
    // Depend on your platform target. (See README.md)
    match env::var("TARGET") {
        Ok(target) => format!("Hey {target}!"),
        Err(_e) => "No TARGET env defined!".to_owned(),
    }
}

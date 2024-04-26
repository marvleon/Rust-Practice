mod question;
use question::*;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    //Router with route to handle GET requests
    let app = Router::new().route("/hello", get(get_questions).fallback(handler_not_found));

    //Address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_questions() -> impl IntoResponse {
    let question = Question::new(
        "1",
        "First Questions",
        "Content of question",
        Some(vec!["faq".to_string()]),
    );
    Json(question) //Return statement 200 OK with JSON serialized data
}

async fn handler_not_found() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

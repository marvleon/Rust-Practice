mod question;
use question::*;
mod questionbase;
use questionbase::*;
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let questionbase = QuestionBase::new();

    //Router with route to handle GET requests
    let app = Router::new()
        .route("/hello", get(get_questions))
        .route("/questions", get(questions))
        .fallback(handler_not_found)
        .with_state(Arc::new(questionbase));

    //Address to serve on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn questions(State(questionbase): State<Arc<QuestionBase>>) -> Response {
    questionbase.into_response()
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

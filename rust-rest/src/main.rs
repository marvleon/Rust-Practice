use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Extension, Json, Router, Server,
};

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct AnswerId(String);

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[derive(Debug)]
enum Error {
    ParseError(String),
    MissingParameters,
    QuestionNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Error::ParseError(e) => (axum::http::StatusCode::BAD_REQUEST, e),
            Error::MissingParameters => (
                axum::http::StatusCode::BAD_REQUEST,
                "Missing required parameters".to_string(),
            ),
            Error::QuestionNotFound => (
                axum::http::StatusCode::NOT_FOUND,
                "Question not found".to_string(),
            ),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// Hanlder for get_questions
async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Arc<RwLock<Store>>>,
) -> Result<Json<Vec<Question>>, Error> {
    let store = store.write().await;
    if !params.is_empty() {
        let start = params
            .get("start")
            .and_then(|v| v.parse::<usize>().ok())
            .ok_or(Error::MissingParameters)?;
        let end = params
            .get("end")
            .and_then(|v| v.parse::<usize>().ok())
            .ok_or(Error::MissingParameters)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(Json(res[start..end].to_vec()))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(Json(res))
    }
}

// Handler to add a new question
async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    //access the Store object first by acquiring a write lock
    let store = store.write().await;

    //access the questions Arc<RwLock<HashMap>> and then acquire a write lock
    let mut questions = store.questions.write().await;

    //insert the question into the HashMap
    questions.insert(question.id.clone(), question);

    //Return a response
    Response::builder()
        .status(StatusCode::CREATED)
        .body("Question added".to_string())
        .unwrap()
}

// Handler to update an existing question
async fn update_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(question_id): Path<QuestionId>,
    Json(updated_question): Json<Question>,
) -> impl IntoResponse {
    // Access the Store object first by acquiring a write lock
    let store = store.write().await;

    // Access the questions Arc<RwLock<HashMap>> and then acquire a write lock
    let mut questions = store.questions.write().await;
    // Update the question in the HashMap
    questions.insert(question_id, updated_question);

    // Return a response
    Response::builder()
        .status(StatusCode::OK)
        .body("Question updated".to_string())
        .unwrap()
}

// Handler to delete a question
async fn delete_question(
    Path(id): Path<String>,
    State(store): State<Arc<RwLock<Store>>>,
) -> impl IntoResponse {
    let store = store.write().await;
    let mut questions = store.questions.write().await;
    if questions.remove(&QuestionId(id)).is_some() {
        Response::builder()
            .status(StatusCode::OK)
            .body("Question deleted".to_string())
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Question not found".to_string())
            .unwrap()
    }
}
#[tokio::main]
async fn main() {
    let store = Store::new();
    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions", post(add_question))
        .route("/questions/:id", put(update_question))
        .route("/questions/:id", delete(delete_question))
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

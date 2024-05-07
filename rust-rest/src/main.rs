use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router, Server,
};

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
//use tower_http::cors::{Any, CorsLayer};

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
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: (Self::init()),
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
// Hanlder for get_questions
async fn get_questions(
    Query(params): Query<HashMap<String, String>>,
    State(store): State<Arc<Mutex<Store>>>,
) -> Result<Json<Vec<Question>>, Error> {
    let store = store.lock().await;

    if params.is_empty() {
        return Err(Error::MissingParameters);
    }

    let start = params
        .get("start")
        .and_then(|v| v.parse::<usize>().ok())
        .ok_or(Error::ParseError("Invalid start parameter".to_string()))?;

    let end = params
        .get("end")
        .and_then(|v| v.parse::<usize>().ok())
        .ok_or(Error::ParseError("Invalid end parameter".to_string()))?;

    if start >= end {
        return Err(Error::ParseError(
            "End parameter must be greater than start".to_string(),
        ));
    }

    let res: Vec<Question> = store.questions.values().cloned().collect();

    if start >= res.len() || end > res.len() {
        return Err(Error::QuestionNotFound);
    }

    Ok(Json(res[start..end].to_vec()))
}

// Handler to add a new question
async fn add_question(
    State(store): State<Arc<Mutex<Store>>>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    //Access the Store object first by acquiring a write lock
    let mut store = store.lock().await;

    //Insert the question into the HashMap
    store.questions.insert(question.id.clone(), question);

    //Return a response
    Response::builder()
        .status(StatusCode::CREATED)
        .body("Question added".to_string())
        .unwrap()
}

// Handler to update an existing question
async fn update_question(
    State(store): State<Arc<Mutex<Store>>>,
    Path(question_id): Path<QuestionId>,
    Json(updated_question): Json<Question>,
) -> impl IntoResponse {
    //Access the Store object first by acquiring a write lock
    let mut store = store.lock().await;

    //Update the question in the HashMap
    if store.questions.contains_key(&question_id) {
        store.questions.insert(question_id, updated_question);
    }

    //Return a response
    Response::builder()
        .status(StatusCode::OK)
        .body("Question updated".to_string())
        .unwrap()
}

//Handler to delete a question
async fn delete_question(
    Path(question_id): Path<QuestionId>,
    State(store): State<Arc<Mutex<Store>>>,
) -> impl IntoResponse {
    let mut store = store.lock().await;

    //Check if the question exists and remove it
    if store.questions.remove(&question_id).is_some() {
        //Return success message
        (
            StatusCode::OK,
            Json(json!({"message": "Question deleted successfully"})),
        )
    } else {
        //Return an error if the question does not exist
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Question not found"})),
        )
    }
}

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(Store::new()));
    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions", post(add_question))
        .route("/questions/:id", put(update_question))
        .route("/questions/:id", delete(delete_question))
        .with_state(store);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

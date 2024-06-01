use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router, Server,
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Question {
    id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Answer {
    id: String,
    content: String,
    question_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct AnswerId(String);

#[derive(Clone)]
struct Store {
    questions: HashMap<String, Question>,
    pool: PgPool,
}

impl Store {
    //constructor for creating an instance of store
    async fn new(pool: PgPool) -> Self {
        let questions = Self::init(&pool).await;
        Store { questions, pool }
    }

    async fn init(pool: &PgPool) -> HashMap<String, Question> {
        let mut questions = HashMap::new();
        //returns a Result<Vec<Question>
        let records = sqlx::query_as!(Question, "SELECT id, title, content, tags FROM questions")
            .fetch_all(pool)
            .await
            .expect("Failed to fetch questions");
        for record in records {
            questions.insert(record.id.clone(), record);
        }
        questions
    }
}

#[derive(Debug)]
enum Error {
    ParseE(String),
    MissingParameters,
    QuestionNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Error::ParseE(e) => (axum::http::StatusCode::BAD_REQUEST, e),
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

//Handler to get ALL questions
async fn questions(State(store): State<Arc<Mutex<Store>>>) -> Result<Json<Vec<Question>>, Error> {
    let store = store.lock().await;
    let questions = store.questions.values().cloned().collect();
    Ok(Json(questions))
}

// Hanlder for get_questions to get paginated questions
async fn get_question(
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
        .ok_or(Error::ParseE("Invalid start parameter".to_string()))?;

    let end = params
        .get("end")
        .and_then(|v| v.parse::<usize>().ok())
        .ok_or(Error::ParseE("Invalid end parameter".to_string()))?;

    if start >= end {
        return Err(Error::ParseE(
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
    sqlx::query!(
        "INSERT INTO questions (id, title, content, tags) VALUES ($1, $2, $3, $4)",
        question.id,
        question.title,
        question.content,
        question.tags.as_deref()
    )
    .execute(&store.pool)
    .await
    .expect("Failed to insert question");

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
    Path(question_id): Path<String>,
    Json(updated_question): Json<Question>,
) -> impl IntoResponse {
    //Access the Store object first by acquiring a write lock
    let mut store = store.lock().await;

    // Execute the SQL update query
    sqlx::query!(
        "UPDATE questions SET title = $2, content = $3, tags = $4 WHERE id = $1",
        question_id,
        updated_question.title,
        updated_question.content,
        updated_question.tags.as_deref()
    )
    .execute(&store.pool)
    .await
    .expect("Failed to update question");

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
    Path(question_id): Path<String>,
    State(store): State<Arc<Mutex<Store>>>,
) -> impl IntoResponse {
    let mut store = store.lock().await;

    // Execute the SQL delete query
    sqlx::query!("DELETE FROM questions WHERE id = $1", question_id)
        .execute(&store.pool)
        .await
        .expect("Failed to delete question");

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
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let store = Store::new(pool).await; // Store::new is an async function and should be awaited
    let shared_store = Arc::new(Mutex::new(store)); // Wrap the store in Mutex, then in Arc

    let app = Router::new()
        .route("/questions", get(questions))
        .route("/question", get(get_question))
        .route("/add_question", post(add_question))
        .route("/update_question/:id", put(update_question))
        .route("/delete_questions/:id", delete(delete_question))
        .with_state(shared_store);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

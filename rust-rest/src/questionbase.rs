use crate::*;
use std::collections::HashMap;
#[derive(Clone)]
pub struct QuestionBase {
    questions: HashMap<String, Question>,
}

impl QuestionBase {
    pub fn new() -> Self {
        QuestionBase {
            questions: Self::init(),
        }
    }

    pub fn init() -> HashMap<String, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json!")
    }
}

impl Default for QuestionBase {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoResponse for &QuestionBase {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.questions)).into_response()
    }
}

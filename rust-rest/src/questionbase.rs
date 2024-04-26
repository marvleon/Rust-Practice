use crate::*;
#[derive(Clone)]
pub struct QuestionBase {
    pub questions: HashMap<String, Question>,
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

impl IntoResponse for &QuestionBase {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.questions)).into_response()
    }
}

impl Default for QuestionBase {
    fn default() -> Self {
        Self::new()
    }
}

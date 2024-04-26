use crate::*;
use std::collections::HashMap;

struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }
    fn init(self) -> Self {
        let question = Question::new(
            "1",
            "How?".to_string(),
            "Please help".to_string(),
            Some(vec!["general".to_string()]),
        );
        self.add_question(question)
    }
    fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
}

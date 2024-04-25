use axum::{routing::get, Router};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

//Custom parsing for converting string to QuestionId
//For extracting QuestionId from strings (user input or data field)
//Returns QuestionId if string is not empty, error if empty
impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

//Func for new instance of Question
//Takes ownership of input parameters and puts them to respective properties of Question struct
impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

#[tokio::main]
async fn main() {
    let hello = axum::get().map(|| format!("Hello, World!!"));
    axum::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

//Implementation of Dipslay trait for struct Question
//Enables custom string formatting of Question instances
//Displays the question struct in a formatted string
impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

//Implementation of Display trait for struct QuestionId
//Allows custom string formatting for QuestionId type
//Outputs the id of the QuestionId (printed or logged)
impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}

use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuestionId(String);

#[derive(Debug, Clone, Serialize)]
pub struct Question {
    id: String,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

//Func for new instance of Question
//Takes ownership of input parameters and puts them to respective properties of Question struct
impl Question {
    pub fn new(id: &str, title: &str, content: &str, tags: Option<Vec<String>>) -> Self {
        let id = id.into();
        let title = title.into();
        let content = content.into();
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}
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

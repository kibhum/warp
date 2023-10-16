use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::Filter;

struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}
#[derive(Debug)]
struct QuestionId(String);

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

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}
impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "id: {}", self.0)
    }
}
impl std::fmt::Debug for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.tags)
    }
}

#[tokio::main]
async fn main() {
    let hello = warp::get().map(|| format!("Hello, World!"));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

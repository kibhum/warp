use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};
// Imports the sqlx Error and
// renames it so there is no
// confusion with our own
// Error enum
use sqlx::error::Error as SqlxError;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    InvalidRange,
    QuestionNotFound,
    // Adds a new error
    // type to our enum,
    // which can hold the
    // actual sqlx error
    DatabaseQueryError(SqlxError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::InvalidRange => write!(f, "Invalid Range"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::DatabaseQueryError(_) => {
                write!(f, "Query could not be executed")
            }
        }
    }
}
impl Reject for Error {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    }
    // else if let Some(_InvalidId) = r.find::<InvalidId>() {
    //     Ok(warp::reply::with_status(
    //         "No valid ID presented".to_string(),
    //         StatusCode::UNPROCESSABLE_ENTITY,
    //     ))
    // }
    else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

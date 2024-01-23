use argon2::Error as ArgonError;
use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;
use tracing::{event, Level};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};
const DUPLICATE_KEY: u32 = 23505;
// Imports the sqlx Error and
// renames it so there is no
// confusion with our own
// Error enum
use sqlx::error::Error as SqlxError;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    WrongPassword,
    ArgonLibraryError(ArgonError),
    // InvalidRange,
    // QuestionNotFound,
    // Adds a new error
    // type to our enum,
    // which can hold the
    // actual sqlx error
    DatabaseQueryError(SqlxError),
    // DatabaseQueryError,
    // ExternalAPIError(ReqwestError),
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
    // In case the HTTP client
    // (Reqwest) returns an error,
    // we create a ClientError
    // enum variant
    ClientError(APILayerError),
    // In case the external API returns a
    // 4xx or 5xx HTTP status code, we
    // have a ServerError variant.
    ServerError(APILayerError),
    Unauthorized,
    CannotDecryptToken,
    MigrationError(sqlx::migrate::MigrateError),
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::ParseError(err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            // Error::InvalidRange => write!(f, "Invalid Range"),
            // Error::QuestionNotFound => write!(f, "Question not found"),
            // Error::DatabaseQueryError(ref err) => {
            //     write!(f, "Query could not be executed: {}", err)
            // }
            Error::DatabaseQueryError(_) => {
                write!(f, "Cannot update, invalid data.")
            }
            // Error::ExternalAPIError(err) => {
            //     write!(f, "Cannot execute: {}", err)
            // }
            Error::ReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            }
            Error::MiddlewareReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            }
            Error::WrongPassword => {
                write!(f, "Wrong password")
            }
            Error::ArgonLibraryError(_) => {
                write!(f, "Cannot verifiy password")
            }
            Error::CannotDecryptToken => write!(f, "Cannot decrypt error"),
            Error::ClientError(err) => {
                write!(f, "External Client error: {}", err)
            }
            Error::ServerError(err) => {
                write!(f, "External Server error: {}", err)
            }
            Error::Unauthorized => write!(f, "No permission to change the underlying resource"),
            Error::MigrationError(_) => write!(f, "Cannot migrate data"),
        }
    }
}
impl Reject for Error {}
impl Reject for APILayerError {}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(crate::Error::DatabaseQueryError(e)) = r.find() {
        // event!(
        //     Level::ERROR,
        //     code = error
        //         .as_database_error()
        //         .unwrap()
        //         .code()
        //         .unwrap()
        //         .parse::<i32>()
        //         .unwrap(),
        //     db_message = error.as_database_error().unwrap().message(),
        //     constraint = error.as_database_error().unwrap().constraint().unwrap()
        // );
        // event!(Level::ERROR, "Database query error");
        // Ok(warp::reply::with_status(
        //     crate::Error::DatabaseQueryError.to_string(),
        //     // "Invalid entity".to_string(),
        //     StatusCode::UNPROCESSABLE_ENTITY,
        // ))
        // Matches against sqlx::Error
        // to see if we have a database
        // error on our hands
        match e {
            sqlx::Error::Database(err) => {
                // If it’s a database
                // error, we know we have a
                // code field. We parse the
                // &str to a i32 so we can
                // compare it to the one
                // we are looking for
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        // If it’s the code we are
                        // looking for, we pass
                        // back a message
                        // that the account
                        // already exists
                        "Account already exsists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    }
    // else if let Some(crate::Error::ExternalAPIError(e)) = r.find() {
    //     event!(Level::ERROR, "{}", e);
    //     Ok(warp::reply::with_status(
    //         "Internal Server Error".to_string(),
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //     ))
    // }
    else if let Some(crate::Error::ReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::MiddlewareReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(crate::Error::ClientError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::ServerError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
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
    } else if let Some(crate::Error::WrongPassword) = r.find() {
        event!(Level::ERROR, "Entered wrong password");
        Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::Unauthorized) = r.find() {
        event!(Level::ERROR, "Not matching account id");
        Ok(warp::reply::with_status(
            "No permission to change underlying resource".to_string(),
            StatusCode::UNAUTHORIZED,
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

#![warn(clippy::all)]
use handle_errors::return_error;
mod routes;
mod store;
mod types;
// use std::io::{Error, ErrorKind};
// use std::str::FromStr;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
// impl Question {
//     fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
//         Question {
//             id,
//             title,
//             content,
//             tags,
//         }
//     }
// }

// #[derive(Debug)]
// struct InvalidId;
// impl Reject for InvalidId {}

// impl std::fmt::Display for Question {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         write!(
//             f,
//             "{}, title: {}, content: {}, tags: {:?}",
//             self.id, self.title, self.content, self.tags
//         )
//     }
// }
// impl std::fmt::Display for QuestionId {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
//         write!(f, "id: {}", self.0)
//     }
// }
// impl std::fmt::Debug for Question {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "{:?}", self.tags)
//     }
// }

#[tokio::main]
async fn main() {
    // env_logger::init();
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    // log::error!("This is an error!");
    // log::info!("This is info!");
    // log::warn!("This is a warning!");

    // Step 1:
    // Add the
    // log level.
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "webdev=info,warp=error".to_owned());

    // let log = warp::log::custom(|info| {
    //     log::info!(
    //         "{} {} {} {:?} from {} with {:?}",
    //         info.method(),
    //         info.path(),
    //         info.status(),
    //         info.elapsed(),
    //         info.remote_addr().unwrap(),
    //         info.request_headers()
    //     );
    // });

    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    // let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    // Step 2: Set
    // the tracing
    // subscriber.
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);
    let get_questions = warp::get()
        .and(warp::path("questions"))
        // Uses path::end to signal that we listen on
        // exactly /question (and not /question/further/
        // params, for example)
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        // .and(id_filter)
        .and_then(routes::question::get_questions)
        // Step 3: Set
        // up logging for
        // custom events.
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_questions request",
                method = %info.method(),
                path = %info.path(),id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        // Adds a String parameter, so
        // the filter is getting triggered for
        // /questions/1234, for example
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        // Extracts the JSON body,
        // which is getting added to
        // the parameters as well
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);
    //Defines the routes variable,
    // which will come in handy later
    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(add_answer)
        .or(delete_question)
        .with(cors)
        // .with(log)
        // Step 4: Set
        // up logging for
        // incoming requests.
        .with(warp::trace::request())
        .recover(return_error);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// impl FromStr for QuestionId {
//     type Err = std::io::Error;
//     fn from_str(id: &str) -> Result<Self, Self::Err> {
//         match id.is_empty() {
//             false => Ok(QuestionId(id.to_string())),
//             true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
//         }
//     }
// }

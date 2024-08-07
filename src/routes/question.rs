use crate::store::Store;
use crate::types::question::{NewQuestion, Question};
use crate::types::{pagination::extract_pagination, pagination::Pagination};
// use handle_errors::Error;
use std::collections::HashMap;
// use tracing::{info, instrument};
use crate::profanity::check_profanity;
// use serde::{Deserialize, Serialize};
use crate::types::account::Session;
use warp::http::StatusCode;

use tracing::{event, info, instrument, Level};

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct APIResponse {
//     message: String,
// }
// #[derive(Deserialize, Serialize, Debug, Clone)]
// struct BadWord {
//     original: String,
//     word: String,
//     deviations: i64,
//     info: i64,
//     #[serde(rename = "replacedLen")]
//     replaced_len: i64,
// }
// #[derive(Deserialize, Serialize, Debug, Clone)]
// struct BadWordsResponse {
//     content: String,
//     bad_words_total: i64,
//     bad_words_list: Vec<BadWord>,
//     censored_content: String,
// }

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
    // id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    // Creates a mutable variable
    // with the default parameter
    // for Pagination
    let mut pagination = Pagination::default();
    // let question = Question::new(
    //     QuestionId::from_str("1").expect("No id provided"),
    //     "First Question".to_string(),
    //     "Content of question".to_string(),
    //     Some(vec!["faq".to_string()]),
    // );

    // match question.id.0.parse::<i32>() {
    //     Err(_) => Err(warp::reject::custom(InvalidId)),
    //     Ok(_) => Ok(warp::reply::json(&question)),
    // }
    // match params.get("start") {
    //     Some(start) => println!("{}", start),
    //     None => println!("No start value"),
    // }
    // let mut start = 0;
    // if let Some(n) = params.get("start") {
    //     start = n.parse::<usize>().expect("Could not parse start");
    // }
    // println!("{}", start);
    // let res: Vec<Question> = store.questions.values().cloned().collect();
    // Ok(warp::reply::json(&res))
    // log::info!("{} Start querying questions", id);
    info!("querying questions");
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
        // log::info!("{} Pagination set {:?}", id, &pagination);
        // info!(pagination = true);
        //     let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        //     let res = &res[pagination.start..pagination.end];
        //     Ok(warp::reply::json(&res))
        // } else {
        // log::info!("No pagination used");
    }
    info!(pagination = false);
    // let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
    // Ok(warp::reply::json(&res))
    // let res: Vec<Question> = match store
    //     .get_questions(pagination.limit, pagination.offset)
    //     .await
    // {
    //     Ok(res) => res,
    //     Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    // };

    // Ok(warp::reply::json(&res))
    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn add_question(
    session: Session,
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    // store
    //     .questions
    //     .write()
    //     .await
    //     .insert(question.id.clone(), question);
    // Ok(warp::reply::with_status("Question added", StatusCode::OK))

    // if let Err(e) = store.add_question(new_question).await {
    //     return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    // }
    // Ok(warp::reply::with_status("Question added", StatusCode::OK))
    // let client = reqwest::Client::new();
    // let res = client
    //     .post("https://api.apilayer.com/bad_words?censor_character=*")
    //     .header("apikey", "8mtoUFCjDvEdyHMxX2MmbEBvPHs8Acm3")
    //     .body("a list with shit words")
    //     .send()
    //     .await
    //     .map_err(|e| handle_errors::Error::ExternalAPIError(e))?
    //     .text()
    //     .await
    //     .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    // println!("{}", res);
    // match store.add_question(new_question).await {
    //     Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
    //     Err(e) => Err(warp::reject::custom(e)),
    // }

    // let client = reqwest::Client::new();
    // let res = client
    //     .post("https://api.apilayer.com/bad_words?censor_character=*")
    //     .header("apikey", "8mtoUFCjDvEdyHMxX2MmbEBvPHs8Acm3")
    //     .body("a list with shit words")
    //     .send()
    //     .await
    //     .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;

    // match res.error_for_status() {
    //     Ok(res) => {
    //         let res = res
    //             .text()
    //             .await
    //             .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    //         println!("{}", res);
    //         match store.add_question(new_question).await {
    //             Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
    //             Err(e) => Err(warp::reject::custom(e)),
    //         }
    //     }
    //     Err(err) => Err(warp::reject::custom(
    //         handle_errors::Error::ExternalAPIError(err),
    //     )),
    // }

    // Checks whether
    // the response
    // status was
    // successful
    // if !res.status().is_success() {
    //     // The status also indicates
    //     // whether it was a client or
    //     // server error
    //     if res.status().is_client_error() {
    //         // The APILayer API
    //         // doesn’t return a
    //         // nice error, so we
    //         // create our own.
    //         let err = transform_error(res).await;
    //         // Returns a
    //         // client or server
    //         // error with our
    //         // APILayerError
    //         // encapsulated
    //         // return Err(handle_errors::Error::ClientError(err));
    //         return Err(warp::reject::custom(handle_errors::Error::ClientError(err)));
    //     } else {
    //         // The APILayer API
    //         // doesn’t return a
    //         // nice error, so we
    //         // create our own.
    //         let err = transform_error(res).await;
    //         // Returns a
    //         // client or server
    //         // error with our
    //         // APILayerError
    //         // encapsulated
    //         // return Err(handle_errors::Error::ServerError(err));
    //         return Err(warp::reject::custom(handle_errors::Error::ServerError(err)));
    //     }
    // }
    // let res = res
    //     .json::<BadWordsResponse>()
    //     .await
    //     .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    // let content = res.censored_content;
    // let question = NewQuestion {
    //     title: new_question.title,
    //     content,
    //     tags: new_question.tags,
    // };
    let account_id = session.account_id;

    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    match store.add_question(question, account_id).await {
        // While we are at it,
        // we return a proper
        // question back to the
        // client instead of just a
        // string and HTTP code
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// async fn transform_error(res: reqwest::Response) -> handle_errors::APILayerError {
//     // Takes a response (which
//     // we know is an error at this
//     // point) and adds a status
//     // code to the message
//     handle_errors::APILayerError {
//         status: res.status().as_u16(),
//         message: res.json::<APIResponse>().await.unwrap().message,
//     }
// }
pub async fn update_question(
    id: i32,
    // We expect the second
    // parameter to be the type
    // Session, since we extract it
    // via the auth middleware.
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    // match store.questions.write().await.get_mut(&QuestionId(id)) {
    //     Some(q) => *q = question,
    //     None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    // }
    // Ok(warp::reply::with_status("Question updated", StatusCode::OK))
    // let res = match store.update_question(question, id).await {
    //     Ok(res) => res,
    //     Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    // };
    // Ok(warp::reply::json(&res))
    // let title = match check_profanity(question.title).await {
    //     Ok(res) => res,
    //     Err(e) => return Err(warp::reject::custom(e)),
    // };
    // let content = match check_profanity(question.content).await {
    //     Ok(res) => res,
    //     Err(e) => return Err(warp::reject::custom(e)),
    // };

    // Uses tokio::spawn to wrap
    // our asynchronous function
    // that returns a future,
    // without awaiting it yet
    // let title = tokio::spawn(check_profanity(question.title));
    // let content = tokio::spawn(check_profanity(question.content));
    // // We can now run both in parallel, returning a
    // // tuple that contains the Result for the title
    // // and one for the content check.
    // let (title, content) = (title.await.unwrap(), content.await.unwrap());
    // // Checks if
    // // both HTTP
    // // calls were
    // // successful
    // if title.is_err() {
    //     return Err(warp::reject::custom(title.unwrap_err()));
    // }
    // if content.is_err() {
    //     return Err(warp::reject::custom(content.unwrap_err()));
    // }

    // Gets account_id out of
    // the Session object to be
    // able to pass a reference
    // to later functions
    let account_id = session.account_id;
    // A newly created
    // store function
    // that checks if
    // the question was
    // originally created
    // by the same
    // account
    if store.is_question_owner(id, &account_id).await? {
        let title = check_profanity(question.title);
        let content = check_profanity(question.content);
        // Instead of the
        // spawn, we don’t
        // have to wrap the
        // function calls
        // separately. We
        // just call them
        // inside the join!
        // macro without
        // any await
        let (title, content) = tokio::join!(title, content);
        if title.is_err() {
            return Err(warp::reject::custom(title.unwrap_err()));
        }
        if content.is_err() {
            return Err(warp::reject::custom(content.unwrap_err()));
        }

        let question = Question {
            id: question.id,
            // title,
            // content,
            title: title.unwrap(),
            content: content.unwrap(),
            tags: question.tags,
        };
        // We now also pass the account_id
        // to the store function, to fill our
        // added account_id column in the
        // database for each new entry.
        match store.update_question(question, id, account_id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        // If the account_id from the Session doesn’t
        // match the one from the database, we
        // return 401 Unauthorized.
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    // match store.questions.write().await.remove(&QuestionId(id)) {
    //     Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
    //     None => Err(warp::reject::custom(Error::QuestionNotFound)),
    // }

    // if let Err(e) = store.delete_question(id).await {
    //     return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    // }
    // Ok(warp::reply::with_status(
    //     format!("Question {} deleted", id),
    //     StatusCode::OK,
    // ))
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        match store.delete_question(id, account_id).await {
            Ok(_) => Ok(warp::reply::with_status(
                format!("Question {} deleted", id),
                StatusCode::OK,
            )),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

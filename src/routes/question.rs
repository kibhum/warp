use crate::store::Store;
use crate::types::question::{NewQuestion, Question};
use crate::types::{pagination::extract_pagination, pagination::Pagination};
// use handle_errors::Error;
use std::collections::HashMap;
// use tracing::{info, instrument};
use warp::http::StatusCode;

use tracing::{event, info, instrument, Level};

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

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "8mtoUFCjDvEdyHMxX2MmbEBvPHs8Acm3")
        .body("a list with shit words")
        .send()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;

    match res.error_for_status() {
        Ok(res) => {
            let res = res
                .text()
                .await
                .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
            println!("{}", res);
            match store.add_question(new_question).await {
                Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        }
        Err(err) => Err(warp::reject::custom(
            handle_errors::Error::ExternalAPIError(err),
        )),
    }
}

pub async fn update_question(
    id: i32,
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
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
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
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

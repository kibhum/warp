use crate::store::Store;
use crate::types::answer::NewAnswer;
// use crate::types::question::QuestionId;
// use std::collections::HashMap;
use crate::profanity::check_profanity;
use warp::http::StatusCode;

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
    // params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // let answer = Answer {
    //     id: AnswerId("1".to_string()),
    //     content: params.get("content").unwrap().to_string(),
    //     question_id: QuestionId(params.get("questionId").unwrap().to_string()),
    // };
    // store
    //     .answers
    //     .write()
    //     .await
    //     .insert(answer.id.clone(), answer);
    // Ok(warp::reply::with_status("Answer added", StatusCode::OK))

    let content = match check_profanity(new_answer.content.to_string()).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };
    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

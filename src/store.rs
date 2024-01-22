use crate::types::{
    account::{Account, AccountId},
    answer::{Answer, AnswerId, NewAnswer},
    question::{NewQuestion, Question, QuestionId},
};
// use std::collections::HashMap;
// use std::sync::Arc;
// use tokio::sync::RwLock;
use handle_errors::Error;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct Store {
    // pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    // pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    pub connection: PgPool,
}
impl Store {
    // pub fn new() -> Self {
    //     Store {
    //         questions: Arc::new(RwLock::new(Self::init())),
    //         answers: Arc::new(RwLock::new(HashMap::new())),
    //     }
    // }

    // pub fn init() -> HashMap<QuestionId, Question> {
    //     let file = include_str!("../questions.json");
    //     serde_json::from_str(file).expect("can't read questions.json")
    // }

    pub async fn is_question_owner(
        &self,
        question_id: i32,
        account_id: &AccountId,
    ) -> Result<bool, Error> {
        // Uses the SELECT
        // query from
        // get_questions
        // and two WHERE
        // clauses to it: id
        // and account_id
        match sqlx::query("SELECT * from questions where id = $1 and account_id = $2")
            .bind(question_id)
            .bind(account_id.0)
            // fetch_optional gives
            // us either None or one
            // answer back.
            .fetch_optional(&self.connection)
            .await
        {
            // Checks whether the
            // result “is some,” and
            // if not, we return false
            Ok(question) => Ok(question.is_some()),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };
        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        // We pass a limit and
        // offset parameter to the
        // function, which indicates
        // if pagination is wanted
        // by the client, and return
        // a vector of questions and
        // a sqlx error type in case
        // something goes wrong.
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, Error> {
        // We write plain SQL
        // via the query function
        // and add the dollar
        // sign ($) and a number
        // for the variables we
        // pass to the query
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            // The bind method replaces a
            // $ + number pair in the SQL
            // query with the variable we
            // specify here.
            .bind(limit)
            // The second
            // bind is our
            // offset variable.
            .bind(offset)
            // If we want to return a question (or
            // all of them) from the query, we use
            // map to go over each returned
            // PostgreSQL row we receive and
            // create a Question out of it.
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            // The fetch_all
            // method executes
            // our SQL statement
            // and returns all the
            // added questions
            // back to us.
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_question(
        &self,
        new_question: NewQuestion,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, content, tags",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .bind(account_id.0)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
        account_id: AccountId,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions
        SET title = $1, content = $2, tags = $3
        WHERE id = $4 AND account_id = $5
        RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .bind(account_id.0)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn delete_question(
        &self,
        question_id: i32,
        account_id: AccountId,
    ) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1 AND account_id = $5")
            .bind(question_id)
            .bind(account_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_answer(
        &self,
        new_answer: NewAnswer,
        account_id: AccountId,
    ) -> Result<Answer, Error> {
        match sqlx::query("INSERT INTO answers (content, question_id) VALUES ($1, $2, $3)")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .bind(account_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn add_account(self, account: Account) -> Result<bool, Error> {
        match sqlx::query(
            "INSERT INTO accounts (email, password)
        VALUES ($1, $2)",
        )
        .bind(account.email)
        .bind(account.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = error.as_database_error().unwrap().message(),
                    constraint = error.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    pub async fn get_account(self, email: String) -> Result<Account, Error> {
        match sqlx::query("SELECT * from accounts where email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(error) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", error);
                Err(Error::DatabaseQueryError(error))
            }
        }
    }
}

use crate::store::Store;
use crate::types::account::{Account, AccountId};
use argon2::{self, Config};
use chrono::prelude::*;
use paseto::v2::local_paseto;
use rand::Rng;
use warp::http::StatusCode;

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    // First checks if the user
    // exists in our database
    match store.get_account(login.email).await {
        // If it does, we
        // verify that the
        // password is the
        // correct one.
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                // If the verification process is successful
                // (the library didn’t fail), we check if the
                // password was
                // indeed verified
                if verified {
                    // and create a
                    // token with the
                    // AccountID in it.
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    // If not, we
                    // create a new
                    // error type called
                    // WrongPassword
                    // and handle this
                    // later in the handleerrors
                    // crate.
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                // If the library
                // fails, we have to
                // send back a 500
                // to the user.
                handle_errors::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    // The argon2 crate will use the salt,
    // which is part of the hash, to verify
    // that the hash from the database is
    // the same as the password from
    // the login process.
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    // let state = serde_json::to_string(&account_id).expect("Failed to serialize”) state");
    // // We issue a token that takes
    // // the AccountID, stringifies it, and
    // // packs it into the paseto token.
    // local_paseto(&state, None, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
    //     .expect("Failed to create token")
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from("RANDOM WORDS WINTER MACINTOSH PC".as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());
    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };
    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// The hash function returns
// a string, the hashed version
// of the clear-text password
pub fn hash_password(password: &[u8]) -> String {
    // The rand function creates
    // s32 random bytes and
    // stores them in a slice.
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    // Argon2 depends on a configuration,
    // and we will use the default set.
    let config = Config::default();
    // With the password, the salt, and
    // the config, we can hash our
    // clear-text password.
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

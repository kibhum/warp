use crate::store::Store;
use crate::types::account::Account;
use argon2::{self, Config};
use rand::Rng;
use warp::http::StatusCode;

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

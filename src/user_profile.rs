use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    pub id: i64,
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct AuthCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
struct AuthResponseUserInfo {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    user: AuthResponseUserInfo,
    token: String,
    centrifugo_token: String,
}

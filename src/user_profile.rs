use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserProfile {
    pub id: i64,
    pub email: String,
}

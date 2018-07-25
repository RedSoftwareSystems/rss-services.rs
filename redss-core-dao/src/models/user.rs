// use serde_json::*;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_name: String,
}
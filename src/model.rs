use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    pub guid: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

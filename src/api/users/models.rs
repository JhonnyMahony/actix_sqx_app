use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl RegisterForm {
    
    pub fn passw_validation(&self) -> bool{
        self.password == self.confirm_password
    }
}
    

#[derive(Serialize, Deserialize, FromRow)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

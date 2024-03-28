//other models
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::{Validate, ValidationError, validate_email};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};

#[derive(Serialize, Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(custom="email_validation")]
    pub email: String,
    #[validate(custom="contains_uppercase")]
    #[validate(custom="len_validation")]
    pub password: String,
    pub confirm_password: String,
}

#[derive(MultipartForm)]
#[multipart(deny_unknown_fields)]
#[multipart(duplicate_field = "deny")]
pub struct UserMultipart{
    pub name: Text<String>,
    pub surname: Text<String>,
    pub phone_number: Text<String>,
    #[multipart(limit = "10 MiB")]
    pub photo: TempFile
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserProfile{
    pub name: String,
    pub surname: String,
    pub phone_number: String,
    pub photo: String
}

#[derive(Serialize, Deserialize, FromRow, Validate)]
pub struct LoginForm {
    #[validate(custom="email_validation")]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    exp: usize,
}


fn email_validation(email: &str) -> Result<(), ValidationError>{
    if validate_email(email){
        Ok(())
    } else {
        Err(ValidationError::new("incorrect email format"))
    }
}

fn len_validation(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8{
        Err(ValidationError::new("password min len is 8"))
    } else{
        Ok(())
    }
}

fn contains_uppercase(password: &str) -> Result<(), ValidationError> {
    if password.chars().any(|c| c.is_uppercase()) {
        Ok(())
    } else {
        Err(ValidationError::new("password must contain uppercase"))
    }
}

fn passwords_match(password: &str, confirmation: &str) -> Result<(), ValidationError> {
    if password == confirmation {
        Ok(())
    } else {
        Err(ValidationError::new("passwords do not match"))
    }
}



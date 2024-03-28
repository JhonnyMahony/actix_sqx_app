use std::io::Read;

use super::auth::auth_token::AuthenticationToken;
use super::models::{LoginForm, RegisterForm, User, UserMultipart, UserProfile};
use crate::errors::AppError;
use crate::utils::{send_email::send_mail, jwt::{encode_token, decode_token}};
use actix_multipart::form::MultipartForm;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use mime::{Mime, IMAGE_BMP, IMAGE_JPEG, IMAGE_PNG};
use sqlx;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid;
use validator::Validate;

#[post("/register")]
pub async fn register_user(
    pool: web::Data<sqlx::PgPool>,
    user: web::Json<RegisterForm>,
) -> Result<impl Responder, AppError> {
    user.validate()?;

    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");
    println!("{}", hashed_password);
    let query_users = "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *";
    let user = sqlx::query_as::<_, User>(query_users)
        .bind(&user.email)
        .bind(&hashed_password)
        .fetch_one(pool.get_ref())
        .await?;

    let query_profiles = "INSERT INTO user_profiles (user_id) VALUES ($1)";
    sqlx::query(query_profiles)
        .bind(user.id)
        .execute(pool.get_ref())
        .await?;
    let url = "http://127.0.0.1:8000/api/v1/users/verify_user";
    send_mail(&user.email, user.id, url).await;

    Ok(HttpResponse::Ok().json("User created successfully"))
}

#[post("/login")]
pub async fn login_user(
    pool: web::Data<sqlx::PgPool>,
    form_data: web::Json<LoginForm>,
) -> Result<impl Responder, AppError> {
    form_data.validate()?;
    let query = "SELECT * FROM users WHERE email = $1";
    let user = sqlx::query_as::<_, User>(query)
        .bind(&form_data.email)
        .fetch_one(pool.get_ref())
        .await?;
    let user_id = match verify(&form_data.password, &user.password.trim()) {
        Ok(is_correct) => {
            if is_correct {
                user.id
            } else {
                return Ok(HttpResponse::Ok().json("incorrect password"));
            }
        }
        Err(e) => return Err(AppError::Unauthorized(e.to_string().into())),
    };

    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let access_token = encode_token(user_id, exp).await;
    return Ok(HttpResponse::Ok().json(access_token));
}

#[put("/update_profile")]
pub async fn update_profile(
    pool: web::Data<sqlx::PgPool>,
    mut form: MultipartForm<UserMultipart>,
    token: AuthenticationToken,
) -> Result<impl Responder, AppError> {
    let file_types: [Mime; 3] = [IMAGE_BMP, IMAGE_JPEG, IMAGE_PNG];
    let dir = "./media/user_profile_photo";
    println!("{}", form.name.to_string());
    let file_type = match &form.photo.content_type {
        Some(photo) => photo,
        None => return Err(AppError::UnprocessableEntity("no type".into())),
    };
    if !file_types.contains(&file_type) {
        return Err(AppError::UnprocessableEntity("invalid file type".into()));
    }
    let filename = match &form.photo.file_name {
        Some(name) => sanitize_filename::sanitize(name),
        None => sanitize_filename::sanitize("file"),
    };
    let photo_path = format!("{}{}-{}", dir, uuid::Uuid::new_v4(), filename);
    let mut saved_file: fs::File = fs::File::create(&photo_path).await?;
    let mut file_contents = Vec::new(); 
    form.photo.file.read_to_end(&mut file_contents)?;
    saved_file.write_all(&file_contents).await?;
    let query = "UPDATE user_profiles SET name = $2, surname = $3, phone_number = $4, photo = $5  WHERE user_id = $1";
    sqlx::query(query)
        .bind(token.id)
        .bind(form.name.to_string())
        .bind(form.surname.to_string())
        .bind(form.phone_number.to_string())
        .bind(photo_path)
        .execute(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json("user profile updated"))
}

#[get("/get_profile")]
pub async fn get_user_profile(
    pool: web::Data<sqlx::PgPool>,
    token: AuthenticationToken,
) -> Result<impl Responder, AppError> {
    let query = "SELECT * FROM user_profiles WHERE user_id = $1";
    let user_profile = sqlx::query_as::<_, UserProfile>(query)
        .bind(token.id)
        .fetch_one(pool.get_ref())
        .await?;
    Ok(HttpResponse::Ok().json(user_profile))
}

#[get("/verify-user/{token}")]
pub async fn confirm_user(pool: web::Data<sqlx::PgPool>, token: web::Path<String>) -> Result<impl Responder, AppError>{
    let token_data = decode_token(&token);
    let query =  "UPDATE users SET is_verified = TRUE WHERE id = $1";
    sqlx::query(query)
        .bind(token_data?.claims.sub)
        .execute(pool.get_ref())
        .await?;
    
    Ok(HttpResponse::Ok().json("user email verified"))
}


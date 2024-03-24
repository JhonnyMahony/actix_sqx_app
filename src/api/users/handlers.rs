use super::models::{LoginForm, RegisterForm, User};
use super::auth::utils::encode_token;
use super::auth::auth_token::AuthenticationToken;
use crate::errors::AppError;
use crate::middleware::jwt_middleware::AuthToken;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx;
use validator::Validate;
use chrono::{Utc, Duration};

#[post("/jwt_send")]
pub async fn jwt_send() -> Result<impl Responder, AppError> {
    let exp: usize = (Utc::now() + Duration::minutes(15)).timestamp() as usize;
    let id: i32 = 1; 
    let access_token = encode_token(id, exp).await;
    Ok(HttpResponse::Ok().json(&access_token))
}

    
#[post("/protected")]
pub async fn jwt_claim(token: web::ReqData<AuthToken>) -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().json(token.id))
}

#[post("/register")]
pub async fn register_user(
    pool: web::Data<sqlx::PgPool>,
    user: web::Json<RegisterForm>,
    token: AuthenticationToken,
) -> Result<impl Responder, AppError> {
    println!("{}", token.id);
    user.validate()?;

    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");
    let query = "INSERT INTO users (email, password) VALUES ($1, $2)";
    sqlx::query(query)
        .bind(&user.email)
        .bind(hashed_password)
        .execute(pool.get_ref())
        .await?;

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

    match verify(&form_data.password, &user.password) {
        Ok(is_correct) => {
            if is_correct {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Ok(HttpResponse::Ok().json("incorrect password"))
            }
        }
        Err(_) => Err(AppError::InternalServerError),
    }
}

#[get("/{id}")]
pub async fn get_users(id: web::Path<u32>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let id = id.into_inner();
    let query = "SELECT * FROM users WHERE id = $1";
    let user = match sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().json("user not found"),
    };

    actix_web::HttpResponse::Ok().json(user)
}

#[post("/")]
pub async fn post_users(pool: web::Data<sqlx::PgPool>, user: web::Json<User>) -> impl Responder {
    let query = "INSERT INTO users (email, password) VALUES ($1, $2)";

    match sqlx::query(query)
        .bind(&user.email)
        .bind(&user.password)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => println!("user created succesfully"),
        Err(_) => return HttpResponse::BadRequest().json("user not found"),
    }

    println!(
        "Received new user: {} with email {}",
        user.password, user.email
    );
    actix_web::HttpResponse::Ok().json("user created")
}

#[delete("/{id}")]
pub async fn delete_users(id: web::Path<u32>, pool: web::Data<sqlx::PgPool>) -> impl Responder {
    let id = id.into_inner();

    let query = "DELETE FROM users WHERE id = $1";
    match sqlx::query(query).bind(id).execute(pool.get_ref()).await {
        Ok(_) => println!("user deleted succesfully"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
    actix_web::HttpResponse::Ok().json("user deleted")
}

#[put("/{id}")]
pub async fn update_users(
    id: web::Path<u32>,
    pool: web::Data<sqlx::PgPool>,
    user: web::Json<User>,
) -> impl Responder {
    let id = id.into_inner();
    let query = "UPDATE users SET email = $1, password = $2 WHERE id = $3";
    match sqlx::query(query)
        .bind(&user.email)
        .bind(&user.password)
        .bind(id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => println!("user created succesfully"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }
    actix_web::HttpResponse::Ok().json("users updated")
}

use crate::{api::users::models::User, errors::AppError};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};

#[get("/user_by/{id}")]
pub async fn get_users(
    id: web::Path<String>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<impl Responder, AppError> {
    let id = id.into_inner();
    let query = "SELECT * FROM users WHERE email = $1";
    let user = match sqlx::query_as::<_, User>(query)
        .bind(id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => return Err(AppError::InternalServerError),
    };

    Ok(actix_web::HttpResponse::Ok().json(user))
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

    actix_web::HttpResponse::Ok().json("user created")
}

#[delete("/{id}")]
pub async fn delete_users(id: web::Path<u32>, pool: web::Data<sqlx::PgPool>) -> Result<impl Responder, AppError> {
    let id = id.into_inner();

    let query = "DELETE FROM users WHERE id = $1";
    sqlx::query(query).bind(id).execute(pool.get_ref()).await?; 
    Ok(actix_web::HttpResponse::Ok().json("user deleted"))
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


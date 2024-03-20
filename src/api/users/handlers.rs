use super::models::{LoginForm, RegisterForm, User};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx;

#[post("/register")]
pub async fn register_user(
    pool: web::Data<sqlx::PgPool>,
    user: web::Json<RegisterForm>,
) -> impl Responder {
    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Failed to hash password");
    let query = "INSERT INTO users (email, password) VALUES ($1, $2)";
    match sqlx::query(query)
        .bind(&user.email)
        .bind(hashed_password)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => return HttpResponse::Ok().json("user created succesfully"),
        Err(_) => return HttpResponse::BadRequest().json("Cant create user"),
    }
}

#[post("/login")]
pub async fn login_user(
    pool: web::Data<sqlx::PgPool>,
    form_data: web::Json<LoginForm>,
) -> impl Responder {
    let query = "SELECT * FROM users WHERE email = $1";
    let user = match sqlx::query_as::<_, User>(query)
        .bind(&form_data.email)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().json("user not found"),
    };

    match verify(&form_data.password, &user.password) {
        Ok(is_correct) => {
            if is_correct {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Ok().json("incorrect password")
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
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

use actix_web::middleware::Logger;
use actix_session::CookieSession;
use actix_web::{App, HttpServer, web, cookie::SameSite};
use env_logger::Env;
use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
mod api;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn establish_connection() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    
    let pool = establish_connection().await;

    let application = move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(CookieSession::signed(&[0; 32]) // Use a secret key for signed cookies
                    .secure(false) // Set to true in production over HTTPS
                    .same_site(SameSite::Strict)
                    .max_age(24 * 60 * 60),)
            .app_data(web::Data::new(pool.clone()))
            .configure(api::users::routes::config)
    };

    HttpServer::new(application)
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

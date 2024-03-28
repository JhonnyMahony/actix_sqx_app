use actix_web::web;
use crate::api::users::handlers;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/users")
            .service(handlers::register_user)
            .service(handlers::login_user)
            .service(handlers::update_profile)
            .service(handlers::get_user_profile)
    );
}

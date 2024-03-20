use actix_web::web;
use crate::api::users::handlers;


pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/users")
            .service(handlers::register_user)
            .service(handlers::login_user)
            //.service(handlers::get_users)
            //.service(handlers::post_users)
            //.service(handlers::update_users)
            //.service(handlers::delete_users)
    );
}

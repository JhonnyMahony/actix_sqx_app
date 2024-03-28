use actix_web::web;
use crate::api::admin::handlers;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/admin")
            .service(handlers::get_users)
            .service(handlers::post_users)
            .service(handlers::update_users)
            .service(handlers::delete_users)
    );
}

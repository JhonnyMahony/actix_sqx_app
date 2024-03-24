use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized, Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    pub id: i32,
}

// Middleware definition remains the same
pub struct JwtValidator;

impl<S, B> Transform<S, ServiceRequest> for JwtValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddleware { service }))
    }
}

pub struct JwtMiddleware<S> {
    service: S,
}

// Extract user ID from token and create `AuthenticationToken` struct
impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = &req.headers().get("Authorization").unwrap().to_str().unwrap();
        if token.starts_with("Bearer ") {
            let token = &token[7..];
            match crate::api::users::auth::utils::decode_token(token) {
                Ok(token) => {
                    let auth_token = AuthToken { id: token.claims.sub };
                    req.extensions_mut().insert(auth_token);
                    return Box::pin(self.service.call(req))
                }
                Err(_) => {
                    return Box::pin(ready(Err(ErrorUnauthorized("Unauthorized"))))
                }
            }
        }
        Box::pin(ready(Err(ErrorUnauthorized("Unauthorized"))))
    }
}

impl FromRequest for AuthToken {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(auth_token) = req.extensions().get::<AuthToken>() {
            ready(Ok(auth_token.clone()))
        } else {
            ready(Err(ErrorUnauthorized("Unauthorized")))
        }
    }
}

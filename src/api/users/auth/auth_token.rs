use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header::HeaderValue, Error as ActixWebError,
    FromRequest, HttpRequest,
};
use jsonwebtoken::{
    errors::Error as JwtError, TokenData
};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use crate::utils::jwt::decode_token;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: i32,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        let authorization_header_option: Option<&HeaderValue> =
            req.headers().get(actix_web::http::header::AUTHORIZATION);

        if authorization_header_option.is_none() {
            return ready(Err(ErrorUnauthorized("No authentication token sent!")));
        }

        let authentication_token: String = authorization_header_option
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string();

        if authentication_token.is_empty() {
            return ready(Err(ErrorUnauthorized(
                "Authentication token has foreign chars!",
            )));
        }

        let token_result: Result<TokenData<Claims>, JwtError> = decode_token(&authentication_token[7..]); 
        match token_result {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.sub,
            })),
            Err(_e) => ready(Err(ErrorUnauthorized("Invalid authentication token sent!"))),
        }
    }
}

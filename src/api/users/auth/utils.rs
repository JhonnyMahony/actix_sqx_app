use std::env;
use jsonwebtoken::{
    encode,
    decode,
    Header,
    EncodingKey,
    DecodingKey,
    Algorithm, Validation, TokenData, errors::Error as JwtError, 
};
use super::auth_token::Claims;

pub async fn encode_token(sub: i32, exp: usize) -> String{
    
    let claim = Claims {sub, exp};
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set"); 
    let token: String = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret_key.as_str().as_ref())
    ).unwrap();
    token
}



pub fn decode_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_str().as_ref()),
        &Validation::new(Algorithm::HS256)
    );
    decoded
    

}

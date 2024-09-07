use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::error::AppError;

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, actix_web::Error> {
    let token = credentials.token();
    match validate_token(token) {
        Ok(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(req)
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}

fn validate_token(_token: &str) -> Result<i32, AppError> {
    // Implement your token validation logic here
    // For now, we'll just return a dummy user_id
    Ok(1)
}
// In src/middleware/auth.rs
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth;

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let token = credentials.token();
    match auth::validate_token(token) {
        Ok(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(req)
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}
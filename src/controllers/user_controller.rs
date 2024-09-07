use crate::models::User;
use crate::services::user_service;
use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

async fn create_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    match user_service::create_user(&pool, &user.username, &user.email, &user.password) {
        Ok(created_user) => HttpResponse::Created().json(UserResponse {
            id: created_user.id,
            username: created_user.username,
            email: created_user.email,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_user(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match user_service::get_user_by_id(&pool, user_id.into_inner()) {
        Ok(user) => HttpResponse::Ok().json(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        }),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/{user_id}", web::get().to(get_user))
    );
}
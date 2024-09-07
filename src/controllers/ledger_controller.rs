use crate::services::ledger_service;
use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub user_id: i32,
    pub amount: BigDecimal,
    pub transaction_type: String,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub id: i32,
    pub user_id: i32,
    pub amount: BigDecimal,
    pub transaction_type: String,
    pub created_at: chrono::NaiveDateTime,
}

async fn create_transaction(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    transaction: web::Json<CreateTransactionRequest>,
) -> impl Responder {
    match ledger_service::create_transaction(&pool, transaction.user_id, &transaction.amount, &transaction.transaction_type) {
        Ok(created_transaction) => HttpResponse::Created().json(TransactionResponse {
            id: created_transaction.id,
            user_id: created_transaction.user_id,
            amount: created_transaction.amount,
            transaction_type: created_transaction.transaction_type,
            created_at: created_transaction.created_at,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_user_transactions(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match ledger_service::get_transactions_by_user_id(&pool, user_id.into_inner()) {
        Ok(transactions) => HttpResponse::Ok().json(transactions.into_iter().map(|t| TransactionResponse {
            id: t.id,
            user_id: t.user_id,
            amount: t.amount,
            transaction_type: t.transaction_type,
            created_at: t.created_at,
        }).collect::<Vec<TransactionResponse>>()),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/transactions")
            .route("", web::post().to(create_transaction))
            .route("/user/{user_id}", web::get().to(get_user_transactions))
    );
}
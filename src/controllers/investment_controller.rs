// investment_controller.rs / creates investment and get investment

use crate::models::{Investment, Case};
use crate::services::investment_service;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateInvestmentRequest {
    pub user_id: i32,
    pub case_id: i32,
    pub amount: f64,
    pub investment_type: String,
}

#[derive(Serialize)]
pub struct InvestmentResponse {
    pub id: i32,
    pub user_id: i32,
    pub case_id: i32,
    pub amount: f64,
    pub investment_type: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct CaseResponse {
    pub id: i32,
    pub lawyer_id: i32,
    pub title: String,
    pub description: String,
    pub funding_goal: f64,
    pub current_funding: f64,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub async fn create_investment(investment: web::Json<CreateInvestmentRequest>) -> impl Responder {
    match investment_service::create_investment(
        investment.user_id,
        investment.case_id,
        investment.amount,
        &investment.investment_type
    ) {
        Ok(created_investment) => HttpResponse::Created().json(InvestmentResponse {
            id: created_investment.id,
            user_id: created_investment.user_id,
            case_id: created_investment.case_id,
            amount: created_investment.amount,
            investment_type: created_investment.investment_type,
            created_at: created_investment.created_at,
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_investments(user_id: web::Path<i32>) -> impl Responder {
    match investment_service::get_investments_by_user_id(user_id.into_inner()) {
        Ok(investments) => HttpResponse::Ok().json(investments.into_iter().map(|i| InvestmentResponse {
            id: i.id,
            user_id: i.user_id,
            case_id: i.case_id,
            amount: i.amount,
            investment_type: i.investment_type,
            created_at: i.created_at,
        }).collect::<Vec<InvestmentResponse>>()),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_available_cases() -> impl Responder {
    match investment_service::get_available_cases() {
        Ok(cases) => HttpResponse::Ok().json(cases.into_iter().map(|c| CaseResponse {
            id: c.id,
            lawyer_id: c.lawyer_id,
            title: c.title,
            description: c.description,
            funding_goal: c.funding_goal,
            current_funding: c.current_funding,
            status: c.status,
            created_at: c.created_at,
            updated_at: c.updated_at,
        }).collect::<Vec<CaseResponse>>()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
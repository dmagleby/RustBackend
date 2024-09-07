use diesel::prelude::*;
use crate::schema::*;
use bigdecimal::BigDecimal;

#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub amount: BigDecimal,
    pub transaction_type: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = cases)]
pub struct Case {
    pub id: i32,
    pub lawyer_id: i32,
    pub title: String,
    pub description: String,
    pub funding_goal: BigDecimal,
    pub current_funding: BigDecimal,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = investments)]
pub struct Investment {
    pub id: i32,
    pub user_id: i32,
    pub case_id: i32,
    pub amount: BigDecimal,
    pub investment_type: String,
    pub created_at: chrono::NaiveDateTime,
}
use crate::models::{Investment, Case};
use crate::schema::{investments, cases};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn create_investment(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
    case_id: i32,
    amount: f64,
    investment_type: &str
) -> Result<Investment, diesel::result::Error> {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");

    let new_investment = Investment {
        id: 0, // This will be ignored and auto-generated
        user_id,
        case_id,
        amount,
        investment_type: investment_type.to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(investments::table)
        .values(&new_investment)
        .get_result(conn)
}

pub fn get_investments_by_user_id(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32
) -> Result<Vec<Investment>, diesel::result::Error> {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");
    
    investments::table
        .filter(investments::user_id.eq(user_id))
        .load::<Investment>(conn)
}

pub fn get_available_cases(
    pool: &Pool<ConnectionManager<PgConnection>>
) -> Result<Vec<Case>, diesel::result::Error> {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");
    
    cases::table
        .filter(cases::status.eq("open"))
        .load::<Case>(conn)
}
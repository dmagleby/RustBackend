use crate::models::Transaction;
use crate::schema::transactions;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use bigdecimal::BigDecimal;

pub fn create_transaction(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32,
    amount: &BigDecimal,
    transaction_type: &str
) -> Result<Transaction, diesel::result::Error> {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");
    
    let new_transaction = Transaction {
        id: 0, // This will be ignored and auto-generated
        user_id,
        amount: amount.clone(),
        transaction_type: transaction_type.to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(transactions::table)
        .values(&new_transaction)
        .get_result(conn)
}

pub fn get_transactions_by_user_id(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32
) -> Result<Vec<Transaction>, diesel::result::Error> {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");
    
    transactions::table
        .filter(transactions::user_id.eq(user_id))
        .load::<Transaction>(conn)
}
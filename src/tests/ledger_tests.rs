#[cfg(test)]
mod tests {
    use crate::services::{user_service, ledger_service};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::PgConnection;
    use dotenv::dotenv;
    use std::env;

    fn setup_test_db() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .build(manager)
            .expect("Failed to create pool")
    }

    #[test]
    fn test_create_transaction() {
        let pool = setup_test_db();
        
        // First, create a test user
        let username = "testuser";
        let email = "testuser@example.com";
        let password = "password";
        let user = user_service::create_user(&pool, username, email, password).expect("Failed to create test user");

        // Create a transaction
        let amount = 100.0;
        let transaction_type = "deposit";
        let result = ledger_service::create_transaction(&pool, user.id, amount, transaction_type);
        
        assert!(result.is_ok());
        
        let transaction = result.unwrap();
        assert_eq!(transaction.user_id, user.id);
        assert_eq!(transaction.amount, amount);
        assert_eq!(transaction.transaction_type, transaction_type);
        
        // Clean up: Delete the test transaction and user
        use crate::schema::{transactions, users};
        use diesel::prelude::*;
        let conn = &mut pool.get().unwrap();
        diesel::delete(transactions::table.filter(transactions::id.eq(transaction.id)))
            .execute(conn)
            .expect("Failed to delete test transaction");
        diesel::delete(users::table.filter(users::id.eq(user.id)))
            .execute(conn)
            .expect("Failed to delete test user");
    }
}
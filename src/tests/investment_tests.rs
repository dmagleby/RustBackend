// src/tests/investment_tests.rs

#[cfg(test)]
mod tests {
    use crate::services::investment_service;
    use crate::services::user_service;
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
    fn test_create_investment() {
        let pool = setup_test_db();
        
        // First, create a test user
        let username = "testuser";
        let email = "testuser@example.com";
        let password = "password";
        let user = user_service::create_user(&pool, username, email, password).expect("Failed to create test user");

        // Create a test case (you might need to implement this in your case_service)
        // For now, we'll assume case_id 1 exists
        let case_id = 1;

        // Now create an investment
        let amount = 1000.0;
        let investment_type = "equity";
        let result = investment_service::create_investment(&pool, user.id, case_id, amount, investment_type);
        
        assert!(result.is_ok());
        
        let investment = result.unwrap();
        assert_eq!(investment.user_id, user.id);
        assert_eq!(investment.case_id, case_id);
        assert_eq!(investment.amount, amount);
        assert_eq!(investment.investment_type, investment_type);
        
        // Clean up: Delete the test investment and user
        use crate::schema::{investments, users};
        use diesel::prelude::*;
        let conn = &mut pool.get().unwrap();
        diesel::delete(investments::table.filter(investments::id.eq(investment.id)))
            .execute(conn)
            .expect("Failed to delete test investment");
        diesel::delete(users::table.filter(users::id.eq(user.id)))
            .execute(conn)
            .expect("Failed to delete test user");
    }
}
#[cfg(test)]
mod tests {
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
    fn test_create_user() {
        let pool = setup_test_db();
        
        let username = "testuser";
        let email = "testuser@example.com";
        let password = "password";

        let result = user_service::create_user(&pool, username, email, password);
        
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        assert_ne!(user.password_hash, password); // Password should be hashed
        
        // Clean up: Delete the test user
        use crate::schema::users::dsl::*;
        use diesel::prelude::*;
        let conn = &mut pool.get().unwrap();
        diesel::delete(users.filter(id.eq(user.id)))
            .execute(conn)
            .expect("Failed to delete test user");
    }
}
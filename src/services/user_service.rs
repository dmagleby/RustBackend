use crate::models::User;
use crate::schema::users;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use bcrypt::{hash, DEFAULT_COST};

pub fn create_user(
    pool: &Pool<ConnectionManager<PgConnection>>,
    username: &str,
    email: &str,
    password: &str
) -> Result<User, diesel::result::Error> {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");
    
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");
    
    let new_user = User {
        id: 0, // This will be ignored and auto-generated
        username: username.to_string(),
        email: email.to_string(),
        password_hash: hashed_password,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user_by_id(
    pool: &Pool<ConnectionManager<PgConnection>>,
    user_id: i32
) -> Result<User, diesel::result::Error> {
    let conn = &mut pool.get().expect("Couldn't get db connection from pool");
    
    users::table.find(user_id).first(conn)
}
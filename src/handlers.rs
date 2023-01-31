use crate::data::DbPool;
use crate::errors::MyError;
use crate::models;
use crate::models::User;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub async fn create_user(login: String, password: String, pool: &DbPool) -> Result<(), MyError> {
    use crate::schema::users::dsl::users;

    let mut connection = pool.get()?;
    let user_uuid = Uuid::new_v4().to_string();
    let password_hash = pwhash::bcrypt::hash(password)?;
    let user = User {
        uuid: user_uuid,
        login,
        hash: password_hash,
    };
    diesel::insert_into(users)
        .values(&user)
        .execute(&mut connection)?;
    Ok(())
}

pub async fn find_user(user_login: String, pool: &DbPool) -> Result<Option<models::User>, MyError> {
    use crate::schema::users::dsl::*;

    let mut connection = pool.get()?;

    let user = users
        .filter(login.eq(user_login))
        .first(&mut connection)
        .optional()?;
    Ok(user)
}

pub async fn find_user_by_uuid(
    user_uuid: String,
    pool: &DbPool,
) -> Result<Option<models::User>, MyError> {
    use crate::schema::users::dsl::*;

    let mut connection = pool.get()?;

    let user = users
        .filter(uuid.eq(user_uuid))
        .first(&mut connection)
        .optional()?;
    Ok(user)
}

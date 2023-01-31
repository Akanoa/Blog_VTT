use crate::data::{DbPool, PostData};
use crate::errors::MyError;
use crate::models::{Post, User};
use crate::{models, schema};
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

pub async fn get_all_posts(pool: &DbPool) -> Result<Vec<PostData>, MyError> {
    use schema::posts;
    use schema::users;

    let mut connection = pool.get()?;

    let join = posts::table.inner_join(users::table);

    let posts_and_author: Vec<(Post, User)> = join.load::<(Post, User)>(&mut connection)?;

    let results = posts_and_author
        .iter()
        .map(From::from)
        .collect::<Vec<PostData>>();

    Ok(results)
}

pub async fn create_post(
    author: User,
    title: String,
    content: String,
    pool: &DbPool,
) -> Result<(), MyError> {
    use crate::schema::posts::dsl::posts;

    let mut connection = pool.get()?;
    let post_uuid = Uuid::new_v4().to_string();
    let post = Post {
        uuid: post_uuid,
        author_uuid: author.uuid,
        title,
        content,
    };
    diesel::insert_into(posts)
        .values(&post)
        .execute(&mut connection)?;

    Ok(())
}

pub async fn get_post_by_uuid(post_uuid: String, pool: &DbPool) -> Result<Option<PostData>, MyError> {
    use schema::posts as schema_posts;
    use schema::users as schema_users;
    use schema::posts::dsl::uuid;

    let mut connection = pool.get()?;

    let join = schema_posts::table.inner_join(schema_users::table);

    let post = join
        .filter(uuid.eq(post_uuid))
        .first::<(Post, User)>(&mut connection)
        .optional()?
        .map(|ref x|From::from(x));

    Ok(post)
}

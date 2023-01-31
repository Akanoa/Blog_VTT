use diesel::prelude::{Insertable, Queryable};
use diesel::{Associations, Identifiable};

use crate::schema::posts;
use crate::schema::users;

#[derive(Identifiable, Queryable, Insertable, Debug)]
#[diesel(primary_key(uuid))]
pub struct User {
    pub uuid: String,
    pub login: String,
    pub hash: String,
}

#[derive(Queryable, Insertable, Associations, Debug)]
#[diesel(belongs_to(User, foreign_key = author_uuid))]
pub struct Post {
    pub uuid: String,
    pub author_uuid: String,
    pub title: String,
    pub content: String,
}

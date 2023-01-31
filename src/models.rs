use diesel::prelude::{Insertable, Queryable};

use crate::schema::users;

#[derive(Queryable, Insertable)]
pub struct User {
    pub uuid: String,
    pub login: String,
    pub hash: String,
}

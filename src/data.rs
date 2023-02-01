use crate::config::Config;
use crate::models::{Post, User};
use actix_web::Result;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, SqliteConnection};
use serde::Serialize;
use tera::{Context, Tera};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// On déclare une structure permettant de stocker le
/// l'état de l'application
pub struct State {
    pub tera: Tera,
    pub db: DbPool,
    pub name: String,
}

#[derive(Serialize)]
pub struct PostData {
    pub(crate) uuid: String,
    pub(crate) title: String,
    pub(crate) author: String,
    pub(crate) content: String,
}

impl From<&(Post, User)> for PostData {
    fn from((post, user): &(Post, User)) -> Self {
        PostData {
            uuid: post.uuid.to_string(),
            title: post.title.to_string(),
            author: user.login.to_string(),
            content: post.content.to_string(),
        }
    }
}

pub fn render_template(tera: &Tera, template_name: &str, context: Context) -> Result<String> {
    let rendered = tera
        .render(template_name, &context)
        // Si cela échoue on renvoie une erreur 500 au client
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(rendered)
}

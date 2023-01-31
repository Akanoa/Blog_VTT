use actix_web::Result;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, SqliteConnection};
use tera::{Context, Tera};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// On déclare une structure permettant de stocker le
/// l'état de l'application
pub struct State {
    pub tera: Tera,
    pub db: DbPool,
}

pub fn render_template(tera: &Tera, template_name: &str, context: Context) -> Result<String> {
    let rendered = tera
        .render(template_name, &context)
        // Si cela échoue on renvoie une erreur 500 au client
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(rendered)
}

use crate::data::{render_template, State};
use crate::handlers::get_all_posts;
use crate::utils::check_user_connected;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};

pub mod auth;
pub mod post;

#[get("/")]
pub async fn index(
    identity: Option<Identity>,
    session: Session,
    state: web::Data<State>,
) -> actix_web::Result<impl Responder> {
    let mut context = tera::Context::new();

    check_user_connected(identity, &state.db, &mut context, session).await?;

    let posts = get_all_posts(&state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    context.insert("posts", &posts);

    // On effectue le rendu du template
    let rendered = render_template(&state.tera, "index.html", context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::Ok().body(rendered))
}

#[get("/login")]
pub async fn login(state: web::Data<State>) -> actix_web::Result<impl Responder> {
    let context = tera::Context::new();
    // On effectue le rendu du template
    let rendered = render_template(&state.tera, "login.html", context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::Ok().body(rendered))
}

#[get("/logout")]
pub async fn logout(
    identity: Option<Identity>,
    state: web::Data<State>,
) -> actix_web::Result<impl Responder> {
    let context = tera::Context::new();

    // Déconnexion
    if let Some(user) = identity {
        user.logout()
    }

    // On effectue le rendu du template
    let rendered = render_template(&state.tera, "index.html", context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .body(rendered))
}

#[get("/register")]
pub async fn register(state: web::Data<State>) -> actix_web::Result<impl Responder> {
    let context = tera::Context::new();
    // On effectue le rendu du template
    let rendered = render_template(&state.tera, "register.html", context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::Ok().body(rendered))
}

use crate::data::{render_template, State};
use crate::handlers::find_user_by_uuid;
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};

pub mod auth;

#[get("/")]
pub async fn index(
    identity: Option<Identity>,
    session: Session,
    state: web::Data<State>,
) -> actix_web::Result<impl Responder> {
    let mut context = tera::Context::new();

    if let Some(identity) = identity {
        // On récupère l'utilisateur par son UUID
        let user = find_user_by_uuid(
            identity
                .id()
                .map_err(actix_web::error::ErrorInternalServerError)?,
            &state.db,
        )
        .await;

        if let Ok(Some(_user)) = user {
            context.insert("session_exists", &true);
        } else {
            // S'il y a une session mais qu'elle n'est pas reconnue
            // on la purge
            session.purge()
        }
    }

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
    Ok(HttpResponse::Ok().body(rendered))
}

#[get("/register")]
pub async fn register(state: web::Data<State>) -> actix_web::Result<impl Responder> {
    let context = tera::Context::new();
    // On effectue le rendu du template
    let rendered = render_template(&state.tera, "register.html", context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::Ok().body(rendered))
}

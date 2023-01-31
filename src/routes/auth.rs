use crate::data::{render_template, State};
use crate::handlers::{create_user, find_user};
use crate::models::User;
use actix_identity::Identity;
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterData {
    login: String,
    password: String,
    password_confirmation: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginData {
    login: String,
    password: String,
}

#[post("/register")]
pub async fn register(
    state: web::Data<State>,
    web::Form(form): web::Form<RegisterData>,
) -> Result<impl Responder> {
    let mut context = tera::Context::new();

    // On vérifie que les mots de passes correspondent
    if form.password != form.password_confirmation {
        context.insert("confirmation_error", &true);

        // On effectue le rendu du template
        let rendered = render_template(&state.tera, "register.html", context)?;
        // Sinon le contenu du template rendu
        return Ok(HttpResponse::BadRequest().body(rendered));
    }

    // On insère l'utilisateur en base de données
    create_user(form.login, form.password, &state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // On effectue le rendu du template
    let rendered = render_template(&state.tera, "login.html", context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/login"))
        .body(rendered))
}

#[post("/login")]
pub async fn login(
    state: web::Data<State>,
    web::Form(form): web::Form<LoginData>,
    request: HttpRequest,
) -> Result<impl Responder> {
    let mut context = tera::Context::new();

    let mut template = "index.html";

    // On recherche l'utilisateur en base
    let user = find_user(form.login, &state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // On vérifie que l'utilisateur existe
    // et possède le bon mot de passe
    match user {
        Some(User { hash, uuid, .. }) => {
            if !pwhash::bcrypt::verify(form.password, &hash) {
                context.insert("bad_password", &true);
                template = "login.html"
            } else {
                Identity::login(&request.extensions(), uuid)
                    .map_err(actix_web::error::ErrorInternalServerError)?;

                // On effectue le rendu du template
                let rendered = render_template(&state.tera, template, context)?;
                return Ok(HttpResponse::SeeOther()
                    .insert_header(("Location", "/"))
                    .body(rendered));
            }
        }
        None => {
            context.insert("bad_password", &true);
            template = "login.html"
        }
    };

    // On effectue le rendu du template
    let rendered = render_template(&state.tera, template, context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::Ok().body(rendered))
}

use crate::data::{render_template, State};
use crate::handlers::{create_post, find_user_by_uuid, get_post_by_uuid};
use crate::utils::check_user_connected;
use actix_identity::{Identity, IdentityExt};
use actix_session::Session;
use actix_web::guard::GuardContext;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use tera::Context;

#[derive(Deserialize)]
pub struct NewPostData {
    title: String,
    content: String,
}

fn connected(ctx: &GuardContext) -> bool {
    ctx.get_identity().is_ok()
}

#[get("/new_post", guard = "connected")]
pub async fn new_post(
    state: web::Data<State>,
    identity: Option<Identity>,
    session: Session,
) -> Result<impl Responder> {
    let mut context = tera::Context::new();
    context.insert("name", &state.name);

    check_user_connected(identity, &state.db, &mut context, session).await?;

    let template = "create_post.html";

    // On effectue le rendu du template
    let rendered = render_template(&state.tera, template, context)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::Ok().body(rendered))
}

#[post("/post", guard = "connected")]
pub async fn publish_post(
    state: web::Data<State>,
    identity: Identity,
    web::Form(form): web::Form<NewPostData>,
) -> Result<impl Responder> {
    let author = find_user_by_uuid(
        identity
            .id()
            .map_err(actix_web::error::ErrorInternalServerError)?,
        &state.db,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?
    .ok_or_else(|| actix_web::error::ErrorForbidden("User not found"))?;

    // On insère l'utilisateur en base de données
    create_post(author, form.title, form.content, &state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Sinon le contenu du template rendu
    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .finish())
}

#[get("/post/{uuid}")]
pub async fn view_post(state: web::Data<State>, uuid: web::Path<String>) -> Result<impl Responder> {
    let post = get_post_by_uuid(uuid.to_string(), &state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match post {
        Some(post) => {
            let mut context = Context::new();
            context.insert("name", &state.name);
            context.insert("post", &post);
            let rendered = render_template(&state.tera, "post.html", context)?;
            // Sinon le contenu du template rendu
            Ok(HttpResponse::Ok().body(rendered))
        }
        None => Ok(HttpResponse::SeeOther()
            .append_header(("Location", "/notexisting"))
            .finish()),
    }
}

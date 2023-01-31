use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use std::env;
use tera::Tera;

#[get("/{name}")]
async fn index(
    name: web::Path<String>, /* Permet de récupérer les paramètres de l'URL */
    state: web::Data<State>, /* Permet de récupérer le state de l'application */
) -> Result<impl Responder> {
    // On créé un context Tera permettant le templating
    let mut context = tera::Context::new();
    // On configure le contexte pour prendre en valeur le paramètre d'URL
    context.insert("name", name.as_str());
    // On effectue le rendu du template
    let rendered = state
        .tera
        .render("hello.html", &context)
        // Si cela échoue on renvoie une erreur 500 au client
        .map_err(actix_web::error::ErrorInternalServerError)?;
    // Sinon le contenu du template rendu
    Ok(HttpResponse::Ok().body(rendered))
}

// Déclaration du "state" de l'application
struct State {
    tera: Tera,
}

// Directive de déclaration du main Actix Web
#[actix_web::main]
// Point d'entrée de Actix Web
async fn main() -> std::io::Result<()> {
    // On récupère le chemin courant
    let pwd = env::current_dir()?
        .to_str()
        // le chemin peut ne pas être de l'UTF-8
        .expect("Bad UTF-8 string")
        .to_string();

    // Création et configuration du moteur de template Tera
    // tous les fichiers html dans le dossiers templates et ses enfants sont
    // utilisables lors du rendu
    let tera = Tera::new(&format!("{}/assets/templates/**/*.html", pwd))
        .expect("Unable to load template engine");

    // Déclaration du serveur HTTP de réponses
    HttpServer::new(move || {
        // Définition du chemin vers les fichiers statique
        let static_path = format!("{}/assets/static", pwd);

        // construction de l'état de l'application
        let state = State {
            // on ajoute le moteur de template a notre état d'application
            tera: tera.clone(),
        };

        App::new()
            // on enregistre le service "index"
            .service(index)
            // on enregistre le service qui sert les fichiers statiques
            .service(actix_files::Files::new("/static", static_path))
            // Ajout de l'état à l'application
            .app_data(web::Data::new(state))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

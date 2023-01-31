use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Responder, Result};
use std::env;

#[get("/")]
async fn index() -> Result<impl Responder> {
    // On récupère le chemin courant
    let pwd = env::current_dir()?
        .to_str()
        // le chemin peut ne pas être de l'UTF-8
        .expect("Bad UTF-8 string")
        .to_string();
    // On récupère les caractères du fichier "index.html"
    // Cette opération peut échouer si le fichier n'existe pas
    let file = NamedFile::open(format!("{}/assets/index.html", pwd))?;
    Ok(file)
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

    // Déclaration du serveur HTTP de réponses
    HttpServer::new(move || {
        // Définition du chemin vers les fichiers statique
        let static_path = format!("{}/assets/static", pwd);

        App::new()
            // on enregistre le service "index"
            .service(index)
            // on enregistre le service qui sert les fichiers statiques
            .service(actix_files::Files::new("/static", static_path))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

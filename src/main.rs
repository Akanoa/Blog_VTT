use actix_web::{App, HttpServer};

// Directive de déclaration du main Actix Web
#[actix_web::main]
// Point d'entrée de Actix Web
async fn main() -> std::io::Result<()> {
    // Déclaration du serveur HTTP de réponses
    HttpServer::new(App::new)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use std::env;

// Import des différents composants
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use blog_from_scratch::config::get_configuration;
use blog_from_scratch::data::State;
use blog_from_scratch::routes;
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tera::Tera;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

// Directive de déclaration du main Actix Web
#[actix_web::main]
// Point d'entrée de Actix Web
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config = get_configuration(args.get(1)).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Bad configuration: {}", e.to_string()),
        )
    })?;

    // Création et configuration du moteur de template Tera
    // tous les fichiers html dans le dossiers templates et ses enfants sont
    // utilisables lors du rendu
    let tera = Tera::new(&format!(
        "{}/assets/templates/**/*.html",
        config.working_directory
    ))
    .expect("Unable to load template engine");

    let manager = ConnectionManager::<SqliteConnection>::new(config.database_path);
    let pool = r2d2::Pool::new(manager).expect("Unable to open database pool");

    let mut connection = pool
        .get()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let secret_key = Key::from(config.session_key.as_bytes());

    // Déclaration du serveur HTTP de réponses
    HttpServer::new(move || {
        // construction de l'état de l'application
        let state = State {
            // on ajoute le moteur de template a notre état d'application
            tera: tera.clone(),
            db: pool.clone(),
        };

        // Définition du chemin vers les fichiers statique
        let static_path = format!("{}/assets/static", config.working_directory);
        App::new()
            // Déclaration du service de fichiers statiques
            .service(actix_files::Files::new("/static", static_path))
            .app_data(web::Data::new(state))
            .service(routes::index)
            .service(routes::login)
            .service(routes::logout)
            .service(routes::register)
            .service(routes::auth::register)
            .service(routes::auth::login)
            .service(routes::post::new_post)
            .service(routes::post::publish_post)
            .service(routes::post::view_post)
            // On enregistre le middleware de gestion d'identité
            .wrap(IdentityMiddleware::default())
            // On enregistre le middleware de gestion de session
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            // On enregistre le logger comme middleware
            .wrap(Logger::default())
    })
    // Déclaration du port d'écoute sur le 8080
    .bind((config.address, config.port))?
    // On démarre le serveur
    .run()
    .await
}

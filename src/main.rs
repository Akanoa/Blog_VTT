use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// Directive de déclaration d'une route "/hello" en verbe GET
// avec paramètres
#[get("/hello/{name}/{number}")]
// Routine de réponses qui accepte un paramètre name de type String
async fn hello(data: web::Path<(String, u32)>) -> impl Responder {
    let (name, number) = data.into_inner();
    // Concaténation avec le paramètre name
    let body = format!("Hello {name} {number}!\n");
    // Création de la réponse en 200
    // avec le body créé
    HttpResponse::Ok().body(body)
}

// Directive de déclaration du main Actix Web
#[actix_web::main]
// Point d'entrée de Actix Web
async fn main() -> std::io::Result<()> {
    // Déclaration du serveur HTTP de réponses
    HttpServer::new(|| {
        App::new()
            // on enregistre le service "hello"
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

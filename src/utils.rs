use actix_identity::Identity;
use actix_session::Session;
use tera::Context;
use crate::data::DbPool;
use crate::handlers::find_user_by_uuid;

pub async fn check_user_connected(identity: Option<Identity>, db: &DbPool, context: &mut Context, session: Session) -> Result<(), actix_web::error::Error> {
    if let Some(identity) = identity {
        // On récupère l'utilisateur par son UUID
        let user = find_user_by_uuid(
            identity
                .id()
                .map_err(actix_web::error::ErrorInternalServerError)?,
            db,
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
    
    Ok(())
}
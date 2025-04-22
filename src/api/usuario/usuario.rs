use actix_web::{web, Scope};

pub fn registrar_rota_usuario() -> Scope
{
    web::scope("/usuario")
        .route("/criar_usuario", web::post().to(super::criar_usuario::criar_usuario_handler))
}



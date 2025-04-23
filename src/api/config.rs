use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use crate::api::usuario::usuario::registrar_rota_usuario;
pub async fn configurar_rotas() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(registrar_rota_usuario())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

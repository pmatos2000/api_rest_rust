use actix_web::{App, HttpServer};
use crate::api::usuario::usuario::registrar_rota_usuario;

pub async fn configurar_rotas() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(registrar_rota_usuario())
    })
        .bind(("0.0.0.0", 8080))? 
        .run()
        .await 
}
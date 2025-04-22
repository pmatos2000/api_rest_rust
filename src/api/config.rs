use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use crate::api::usuario::usuario::registrar_rota_usuario;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub async fn configurar_rotas(pool: &PgPool) -> std::io::Result<()> {
    let state = AppState {
        pool: pool.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(registrar_rota_usuario())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

use actix_web::{App, HttpServer};

mod api;
mod aplicacao;
mod dominio;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::usuario::usuario::registrar_rota())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
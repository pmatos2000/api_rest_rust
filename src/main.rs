use dotenv::dotenv;
use crate::config::migration::executar_migracao;
use crate::api::config::configurar_rotas;

mod api;
mod aplicacao;
mod dominio;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    
    executar_migracao().await;
    
    configurar_rotas().await?;

    Ok(())
}
use dotenv::dotenv;
use crate::config::migration::executar_migracao;
use crate::api::config::configurar_rotas;
use crate::config::db_pool::criar_pool;

mod api;
mod aplicacao;
mod dominio;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    
    let pool = criar_pool().await;
    
    executar_migracao(&pool).await;
    configurar_rotas(&pool).await?;

    Ok(())
}
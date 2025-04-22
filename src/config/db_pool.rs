use once_cell::sync::Lazy;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub async fn criar_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("A variável DATABASE_URL não está configurada");

    let pool = PgPool::connect(&database_url).await
        .expect("Falha ao conectar ao banco de dados");
    
    pool
}

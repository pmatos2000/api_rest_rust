use once_cell::sync::Lazy;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub static POOL: Lazy<Arc<PgPool>> = Lazy::new(|| {
    let database_url = std::env::var("DATABASE_URL")
        .expect("A variável DATABASE_URL não está configurada");

    Arc::new(PgPool::connect_lazy(&database_url)
        .expect("Falha ao conectar ao banco de dados"))
});
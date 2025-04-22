use std::process;
use sqlx::PgPool;

pub async fn executar_migracao(pool: &PgPool) {
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            println!("Migrações aplicadas com sucesso!");
        }
        Err(err) => {
            eprintln!("Erro ao aplicar migrações: {}", err);
            process::exit(1);
        }
    }
}
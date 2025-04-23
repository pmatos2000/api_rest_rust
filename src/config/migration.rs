use std::process;
use crate::config::db_pool::POOL;

pub async fn executar_migracao() {
    let pool = POOL.clone();
    match sqlx::migrate!("./migrations").run(&*pool).await {
        Ok(_) => {
            println!("Migrações aplicadas com sucesso!");
        }
        Err(err) => {
            eprintln!("Erro ao aplicar migrações: {}", err);
            process::exit(1);
        }
    }
}
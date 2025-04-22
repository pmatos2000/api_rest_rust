use super::db_pool::POOL;
use std::process;

pub async fn executar_migracao() {

    match sqlx::migrate!("./migrations").run(&**POOL).await {
        Ok(_) => {
            println!("Migrações aplicadas com sucesso!");
        }
        Err(err) => {
            eprintln!("Erro ao aplicar migrações: {}", err);
            process::exit(1);
        }
    }
}
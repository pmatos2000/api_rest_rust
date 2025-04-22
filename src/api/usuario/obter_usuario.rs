use actix_web::{web, HttpResponse, Responder};

pub async fn _obter_usuario_handler(_body: web::Json<(String, u8)>) -> impl Responder
{
    HttpResponse::Ok()
}
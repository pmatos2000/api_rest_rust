use actix_web::{web, HttpResponse, Responder};
use crate::aplicacao::criar_usuario_aplicacao::CriarUsuarioAplicacao;
use crate::dominio::aplicacao::interface::criar_usuario_aplicacao::TCriarUsuarioAplicacao; 
pub async fn criar_usuario_handler(body: web::Json<(String, u8)>) -> impl Responder {
    let (nome, idade) = body.into_inner();
    let usuario = CriarUsuarioAplicacao::criar_usuario(nome, idade);
    HttpResponse::Ok().json(usuario)
}
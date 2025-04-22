use crate::dominio::aplicacao::interface::criar_usuario_aplicacao::TCriarUsuarioAplicacao;
use crate::dominio::servico::dto::usuario::UsuarioDto;

pub struct CriarUsuarioAplicacao;

impl TCriarUsuarioAplicacao for CriarUsuarioAplicacao {
    fn criar_usuario(nome: String, idade: u8) -> UsuarioDto {
        UsuarioDto { nome, idade }
    }
}
use crate::dominio::servico::dto::usuario::UsuarioDto;

pub trait TCriarUsuarioAplicacao {
    fn criar_usuario(nome: String, idade: u8) -> UsuarioDto;
}

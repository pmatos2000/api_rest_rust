use crate::dominio::servico::dto::usuario::UsuarioDto;

pub trait _TObterUsuarioAplicacao {
    fn _obter_usuario(id: u64) -> UsuarioDto;
}
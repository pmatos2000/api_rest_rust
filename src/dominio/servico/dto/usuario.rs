#[derive(serde::Serialize)]
pub struct UsuarioDto {
    pub nome: String,
    pub idade: u8,
}
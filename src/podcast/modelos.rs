use serde::Serialize;

#[derive(Serialize)]
pub struct Podcast {
    pub error_critico: Option<String>,
    pub errores: Vec<String>,
    pub info: Info,
    pub programas: Vec<Programa>,
}

#[derive(Serialize)]
pub struct Info {
    pub nombre: Option<String>,
    pub descripcion: Option<String>,
    pub imagen: Option<String>,
}

#[derive(Serialize)]
pub struct Programa {
    pub titulo: Option<String>,
    pub descripcion: Option<String>,
    pub url: Option<String>,
    pub descarga: Option<String>,
}

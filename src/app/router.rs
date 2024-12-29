use std::{env::current_exe, path::PathBuf};

use axum::{routing::get, Router};
use axum_embed::ServeEmbed;
use reqwest::Client;
use rust_embed::RustEmbed;

use crate::cliente_http;

use super::{descargar, ivoox_url};

#[derive(RustEmbed, Clone)]
#[folder = "frontend/dist"]
struct ArchivosEstaticos;

#[derive(Clone)]
pub struct App {
    pub http_client: Client,
    pub ruta_raiz: PathBuf,
}

fn obtener_ruta_raiz() -> PathBuf {
    current_exe()
        .expect("ERROR FATAL: no se ha podido recuperar la ruta del ejecutable")
        .parent()
        .expect("no se ha podido recuperar la ruta raíz")
        .to_path_buf()
}

pub fn crear_web_app() -> Router {
    let serve_static = ServeEmbed::<ArchivosEstaticos>::new();

    let app = App {
        http_client: cliente_http::crear::nuevo_cliente_http(),
        ruta_raiz: obtener_ruta_raiz(),
    };

    Router::new()
        .nest_service("/", serve_static)
        .route("/api/podcast", get(ivoox_url::buscar_programas))
        .route("/api/descargar", get(descargar::gestionar_descarga))
        .with_state(app)
}

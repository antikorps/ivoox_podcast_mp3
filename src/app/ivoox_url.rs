use axum::{
    extract::{Query, State},
    http::Response,
};
use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde::Deserialize;

use crate::podcast::analizar::gestionar_fases_creacion;

use super::router::App;

#[derive(Deserialize)]
pub struct UrlQuery {
    pub url: String,
}

pub async fn buscar_programas(
    State(estado): State<App>,
    solicitud: Query<UrlQuery>,
) -> Response<String> {
    // let res = include_str!("resumen.json");
    // Response::builder()
    //     .header(CONTENT_TYPE, "application/json")
    //     .body(res.to_string())
    //     .unwrap()

    match gestionar_fases_creacion(&solicitud.url, &estado.http_client).await {
        Ok(ok) => Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .body(ok.to_string())
            .unwrap(),
        Err(error) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(CONTENT_TYPE, "application/json")
            .body(error.to_string())
            .unwrap(),
    }
}

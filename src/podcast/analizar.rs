use reqwest::Client;

use super::capitulos;

pub async fn gestionar_fases_creacion(url: &str, cliente: &Client) -> Result<String, String> {
    capitulos::buscar_capitulos_paginados(url, cliente).await
}

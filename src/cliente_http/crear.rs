use std::time::Duration;

use reqwest::{header, Client};

pub fn nuevo_cliente_http() -> Client {
    let mut cabeceras = header::HeaderMap::new();
    cabeceras.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:131.0) Gecko/20100101 Firefox/131.0",
        ),
    );

    reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(21))
        .default_headers(cabeceras)
        .cookie_store(true)
        .build()
        .expect("ERROR FATAL: no se ha podido crear el client http")
}

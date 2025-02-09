use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use super::modelos::{Info, Podcast, Programa};

fn extraer_titulo(capitulo: &ElementRef<'_>) -> Option<String> {
    let selector_enlace = Selector::parse("a").unwrap();
    let mut busqueda_enlaces = capitulo.select(&selector_enlace);
    if busqueda_enlaces.clone().count() == 0 {
        return None;
    };
    let titulo: String = busqueda_enlaces.next().unwrap().text().collect();
    Some(titulo.as_str().trim().to_string())
}

fn extraer_url(capitulo: &ElementRef<'_>) -> Option<String> {
    let selector_enlace = Selector::parse("a").unwrap();
    let mut busqueda_enlaces = capitulo.select(&selector_enlace);
    if busqueda_enlaces.clone().count() == 0 {
        return None;
    };
    let enlace_titulo = busqueda_enlaces.next().unwrap();
    enlace_titulo.attr("href").map(|ok| format!("https://www.ivoox.com{}", ok))
}

fn extraer_descarga(url: &Option<String>) -> Option<String> {
    if url.is_none() {
        return None;
    }
    // https://www.ivoox.com/programa-00-presentacion-audios-mp3_rf_26193701_1.html
    // https://www.ivoox.com/programa-00-presentacion-formatos_mf_26193701_feed_1.mp3
    Some(
        url.to_owned()
            .unwrap()
            .replace("-audios-mp3_rf_", "-formatos_mf_")
            .replace("_1.html", "_feed_1.mp3"),
    )
}

fn extraer_descripcion(capitulo: &ElementRef<'_>) -> Option<String> {
    let selector_descripcion = Selector::parse("div.description").unwrap();
    let mut busqueda_descripcion = capitulo.select(&selector_descripcion);
    if busqueda_descripcion.clone().count() == 0 {
        return None;
    };
    let descripcion: String = busqueda_descripcion.next().unwrap().text().collect();
    Some(descripcion.as_str().trim().to_string())
}

fn extraer_informacion_capitulo(capitulo: ElementRef<'_>) -> Programa {
    let titulo = extraer_titulo(&capitulo);
    let url = extraer_url(&capitulo);
    let descarga = extraer_descarga(&url);
    let descripcion = extraer_descripcion(&capitulo);
    Programa {
        titulo,
        url,
        descarga,
        descripcion,
    }
}

fn verificar_url_programa(url: &str) -> Result<(), String> {
    // https://www.ivoox.com/podcast-rock-entre-amigos_sq_f1579217_1.html
    if !url.contains("_sq_") {
        return Err(String::from(
            "la url no parece válida, no se ha encontrado _sq_",
        ));
    }
    if !url.ends_with("_1.html") {
        return Err(String::from(
            "la url no parece válida, no termina en 1.html",
        ));
    }

    Ok(())
}

fn nombre_descripcion_img_podcast(html: &Html) -> Info {
    let selector_imagen_podcast = Selector::parse(".image-wrapper img").unwrap();
    let mut info = Info {
        nombre: None,
        descripcion: None,
        imagen: None,
    };
    let mut busqueda_imagen_podcast = html.select(&selector_imagen_podcast);
    if busqueda_imagen_podcast.clone().count() > 0 {
        let imagen_podcast = busqueda_imagen_podcast.next().unwrap();
        match imagen_podcast.attr("alt") {
            None => (),
            Some(ok) => info.nombre = Some(ok.to_owned()),
        }
        match imagen_podcast.attr("data-lazy-src") {
            None => (),
            Some(ok) => {
                // https://img-static.ivoox.com/index.php?w=175&amp;h=175&amp;url=https://static-1.ivoox.com/canales/0/4/f/f/04ffe1ed47eb8969977fcc13ef525e49_XXL.jpg
                let partes = ok.split("https://").collect::<Vec<&str>>();
                if partes.len() == 3 {
                    info.imagen = Some(format!("https://{}", partes[2]))
                }
            }
        }
    }

    let selector_descripcion = Selector::parse(r#"meta[property="og:description"]"#).unwrap();
    let mut busqueda_descripcion = html.select(&selector_descripcion);
    if busqueda_descripcion.clone().count() > 0 {
        let descripcion = busqueda_descripcion.next().unwrap();
        match descripcion.attr("content") {
            None => (),
            Some(ok) => info.descripcion = Some(ok.to_owned()),
        }
    }
    info
}

pub async fn buscar_capitulos_paginados(url: &str, cliente: &Client) -> Result<String, String> {
    let mut indice = 0;
    let url_sin_pagina = url.strip_suffix("_1.html").unwrap();

    let mut podcast = Podcast {
        error_critico: None,
        errores: Vec::new(),
        info: Info {
            nombre: None,
            descripcion: None,
            imagen: None,
        },
        programas: Vec::new(),
    };

    if let Err(error) = verificar_url_programa(url) {
        podcast.errores.push(error);
    };

    if podcast.errores.is_empty() {
        let mut errores_seguidos = 0;
        loop {
            indice += 1;
            println!("Buscando programas en página {} para {}", indice, url);
            if errores_seguidos > 3 {
                let mensaje_error = String::from("se han producido demasiados errores, no se puede garantizar la extracción de todos los capítulos");
                podcast.errores.push(mensaje_error);
                break;
            }
            let endpoint = format!("{}_{}.html", url_sin_pagina, indice);

            let res = match cliente.get(&endpoint).send().await {
                Err(error) => {
                    let mensaje_error = format!(
                        "ha fallado el envío de la peticion a {} {}",
                        endpoint, error
                    );
                    errores_seguidos += 1;
                    podcast.errores.push(mensaje_error);
                    continue;
                }
                Ok(ok) => ok,
            };

            if res.status() != 200 {
                let mensaje_error = format!(
                    "la peticion a {} ha devuelto un status code incorrecto {}",
                    endpoint,
                    res.status()
                );
                podcast.errores.push(mensaje_error);
                errores_seguidos += 1;
                continue;
            }

            let texto = match res.text().await {
                Err(error) => {
                    let mensaje_error = format!(
                        "ha fallado la lectura de la respuesta de {} {}",
                        endpoint, error
                    );
                    podcast.errores.push(mensaje_error);
                    errores_seguidos += 1;
                    continue;
                }
                Ok(ok) => ok,
            };

            let html = Html::parse_fragment(&texto);
            if indice == 1 {
                podcast.info = nombre_descripcion_img_podcast(&html);
            }

            let selector_capitulos =
                Selector::parse("div.pl-1.pl-sm-2.flex-grow-1.align-items-stretch").unwrap();
            let selectores_capitulos = html.select(&selector_capitulos);
            if selectores_capitulos.clone().count() == 0 {
                break;
            }
            for capitulo in selectores_capitulos {
                let info_capitulo = extraer_informacion_capitulo(capitulo);
                podcast.programas.push(info_capitulo);
            }
        }
    }

    match serde_json::to_string(&podcast) {
        Err(error) => {
            let error_formateado_json = error.to_string().replace("\"", "");
            let mensaje_error = format!(
                r#"{{"error_critico": "ha fallado la serialización en json del análisis {}"}}"#,
                error_formateado_json
            );

            Err(mensaje_error)
        }
        Ok(ok) => Ok(ok),
    }
}

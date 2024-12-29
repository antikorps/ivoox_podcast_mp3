use std::{env, fs::File, io::Write, path::PathBuf, sync::Arc};

use super::router::App;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{
    future::join_all,
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct SolicitudDescarga {
    pub identificador: String,
    pub nombre: String,
    pub archivo: String,
    pub descarga: String,
    pub progreso: String,
    pub finalizado: bool,
}

impl SolicitudDescarga {
    fn notificar_exito_descarga(&self) {
        println!("{}", self.progreso)
    }
}

pub async fn gestionar_descarga(ws: WebSocketUpgrade, State(estado): State<App>) -> Response {
    ws.on_upgrade(|socket| manejador_socket(socket, estado))
}

fn determinar_descargas_simultaneas() -> usize {
    let variable_descargas = match env::var("IVOOX_PODCAST_DESCARGAS") {
        Err(_) => return 1 as usize,
        Ok(ok) => ok,
    };
    match variable_descargas.parse::<usize>() {
        Err(_) => return 1 as usize,
        Ok(ok) => return ok,
    }
}

async fn enviar_progreso(
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    estado_descarga: &SolicitudDescarga,
) {
    let mensaje_ws_serializado = match serde_json::to_string(estado_descarga) {
        Ok(msg) => msg,
        Err(e) => {
            eprintln!("error serializando estado_descarga: {}", e);
            return;
        }
    };

    match sender
        .lock()
        .await
        .send(Message::Text(mensaje_ws_serializado))
        .await
    {
        Err(error) => {
            eprintln!("error enviando mensaje_ws_serializado: {}", error);
        }
        Ok(_) => (),
    }
}

async fn manejar_error_descarga<'a>(
    mensaje_error: String,
    sender: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    estado_descarga: &mut SolicitudDescarga,
) {
    estado_descarga.progreso = mensaje_error;
    estado_descarga.finalizado = true;
    eprintln!("{:#?}", estado_descarga);
    enviar_progreso(sender, estado_descarga).await;
}

pub async fn descargar_archivo_progreso(
    cliente: &Client,
    sender: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    descarga: &SolicitudDescarga,
    ruta_raiz: &PathBuf,
) {
    let ruta_descarga = ruta_raiz.join(&descarga.archivo).display().to_string();
    let mut estado_descarga = SolicitudDescarga {
        identificador: descarga.identificador.to_owned(),
        nombre: descarga.nombre.to_owned(),
        archivo: ruta_descarga,
        descarga: descarga.descarga.to_owned(),
        progreso: String::from("Preparando descarga, por favor, espera..."),
        finalizado: false,
    };

    let res = match cliente.get(&estado_descarga.descarga).send().await {
        Ok(ok) => ok,
        Err(error) => {
            let mensaje_error = format!(
                "❌ no ha podido descargarse el programa, fallo en la petición {}",
                error
            );
            manejar_error_descarga(mensaje_error, &sender, &mut estado_descarga).await;
            return;
        }
    };

    if !res.status().is_success() {
        let mensaje_error = format!(
            "❌ no ha podido descargarse el programa, se ha recibido un status code incorrecto {}",
            res.status()
        );
        manejar_error_descarga(mensaje_error, &sender, &mut estado_descarga).await;
        return;
    }

    let mut tamaño = 0 as u64;
    if res.content_length().is_some() {
        tamaño = res.content_length().unwrap();
    }

    let mut archivo = match File::create(&estado_descarga.archivo) {
        Ok(ok) => ok,
        Err(error) => {
            let mensaje_error = format!("❌ no ha podido descargarse el programa, no ha podido crearse el archivo de destino {}", error);
            manejar_error_descarga(mensaje_error, &sender, &mut estado_descarga).await;
            return;
        }
    };
    let mut descargado: u64 = 0;
    let mut stream = res.bytes_stream();
    while let Some(stream_bytes) = stream.next().await {
        let descarga_bytes = match stream_bytes {
            Ok(ok) => ok,
            Err(error) => {
                let mensaje_error = format!("❌ no ha podido descargarse el programa, ha fallado la lectura de bytes del archivo {}", error);
                manejar_error_descarga(mensaje_error, &sender, &mut estado_descarga).await;
                return;
            }
        };
        match archivo.write_all(&descarga_bytes) {
            Ok(_) => (),
            Err(error) => {
                let mensaje_error = format!("❌ no ha podido descargarse el programa, ha fallado la escritura de bytes del archivo {}", error);
                manejar_error_descarga(mensaje_error, &sender, &mut estado_descarga).await;
                return;
            }
        };

        descargado += descarga_bytes.len() as u64;
        if tamaño != 0 {
            let porcentaje = (descargado as f64 / tamaño as f64) * 100.0;
            estado_descarga.progreso = format!("⏳ {:.2}% descargado", porcentaje)
        } else {
            estado_descarga.progreso =
                format!("⏳ {} bytes descargados de tamaño desconocido", descargado);
        }
        enviar_progreso(&sender, &estado_descarga).await;
    }

    estado_descarga.progreso = format!(
        "✅ programa descargado correctamente en {}",
        estado_descarga.archivo
    );
    estado_descarga.finalizado = true;
    enviar_progreso(&sender, &estado_descarga).await;
    estado_descarga.notificar_exito_descarga();
}

async fn manejador_socket(socket: WebSocket, estado: App) {
    let cliente = estado.http_client;
    let (sender, mut receiver) = socket.split();
    let sender_seguro = Arc::new(Mutex::new(sender));

    while let Some(ws_mensaje) = receiver.next().await {
        let mensaje = match ws_mensaje {
            Err(error) => {
                eprintln!("{}", error);
                return;
            }

            Ok(ok) => ok,
        };
        let texto = match mensaje.into_text() {
            Err(error) => {
                eprintln!("{}", error);
                return;
            }

            Ok(ok) => ok,
        };

        let solicitud: Vec<SolicitudDescarga> = match serde_json::from_str(&texto) {
            Err(error) => {
                eprintln!("{}", error);
                return;
            }

            Ok(ok) => ok,
        };

        for lote in solicitud.chunks(determinar_descargas_simultaneas()) {
            let mut futuros = Vec::new();
            for descarga in lote {
                let sender_clonado = Arc::clone(&sender_seguro);
                futuros.push(descargar_archivo_progreso(
                    &cliente,
                    sender_clonado,
                    descarga,
                    &estado.ruta_raiz,
                ));
            }
            join_all(futuros).await;
        }
    }
}

use std::env;

mod app;
mod cliente_http;
mod podcast;

#[tokio::main]
async fn main() {
    let web_app_router = app::router::crear_web_app();

    for mut puerto in 3000..8000 {
        let mut usar_env_var_puerto = false;
        match env::var("IVOOX_PODCAST_PORT") {
            Err(_) => (),
            Ok(var) => match var.parse::<i32>() {
                Err(error) => {
                    eprintln!("IVOOX_PODCAST_PORT es incorrecto, debe ser un número: {error}");
                }
                Ok(ok) => {
                    puerto = ok;
                    usar_env_var_puerto = true;
                }
            },
        }
        let direccion = format!("0.0.0.0:{puerto}");
        match tokio::net::TcpListener::bind(direccion).await {
            Err(error) => {
                if usar_env_var_puerto {
                    panic!("no se ha podido iniciar la aplicación en el puerto señalado en IVOOX_PODCAST_PORT {error}");
                }
            }

            Ok(ok) => {
                println!(
                    "\nAplicación iniciada en:\n=======================\nhttp://localhost:{}\n\n",
                    puerto
                );
                axum::serve(ok, web_app_router.clone()).await.unwrap();
            }
        }
    }
}

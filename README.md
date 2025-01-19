# Ivoox Podcast a MP3
Descarga en mp3 de manera masiva los programas de tu podcast favorito en Ivoox.
### ATENCIÓN:
Las URL de descarga que proporcionan a los podcast categorizados como "Ivoox Originals" son solo un pequeño fragmento. Para este tipo de podcasts, la única utilidad del programa podría ser la creación de la hoja de cálculo con todos los episodios para facilitar búsquedas. 
## Ejecución e instalación
- Descarga el archivo correspondiente a tu sistema operativo desde el espacio "Releases" del repositorio.
- Dependiendo del sistema operativo tal vez necesites conceder permisos de ejecución al binario.
- Ejecuta el archivo, no necesita otras dependencias.
- Observa la dirección que aparece en la terminal y accede desde el navegador para utilizar la aplicación web (por defecto, intenta ejecutarse en el puerto 3000, por lo que puede accederse desde http://localhost:3000)
## Uso
En el formulario de búsqueda debe incorporarse la URL de la página principal del podcast. Esta página principal es en la que aparecen los programas paginados en pequeñas tarjetas:

![Captura de la página principal de un programa](https://i.imgur.com/z2c3MKQ.png)

Una vez que haya acabado la búsqueda de programas selecciona los programas en los que estés interesado y al hacerlo aparecerá el botón de "Descargar"

**Descargar listado** te puede ser de mucha utilidad, te generará una hoja de cálculo (Excel, .xslx) donde encontrarás toda la información de los programas, por lo que podrías copiar y pegar las URL que te interesen en un gestor de descargas y aprovecharte de todas sus ventajas (resumen de descargas, etc.)

![Captura de pantalla tras finalizar la búsqueda](https://i.imgur.com/9cpIHoH.png)

### Información para desarrolladores
El frontend y el backend se gestionan de manera independiente. Para aprovechar las ventajas del "hot reloaded" en Svelte recuerda configurar el reverse proxy en **vite.config.js** para que apunte al puerto en el que se está ejecutando el backend. 
Por supuesto, antes de compilar, es necesaraio el **build** del frontend en la carpeta **dist** que es la embeberá el ejecutable.

Para compilar:

```bash
cd frontend/ && npx prettier --write . && npm run build && cd .. && cargo run --release
```

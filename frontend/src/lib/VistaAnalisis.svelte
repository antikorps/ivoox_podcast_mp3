<script lang="ts">
  import type { InfoDescarga, WebApp } from "../modelos";
  import { utils, writeFile } from "xlsx";

  type Props = {
    webApp: WebApp;
  };
  let { webApp = $bindable() }: Props = $props();

  function volverBuscar() {
    webApp.descargando = false;
    webApp.descargas = {};
    webApp.numeroDescargas = 0;
    webApp.podcast = null;
  }

  function descargarListado() {
    const programasData = webApp.podcast.programas.map((programa, index) => ({
      Índice: index + 1,
      Título: programa.titulo || "",
      Descripción: programa.descripcion || "",
      URL: programa.url || "",
      Descarga: programa.descarga || "",
    }));

    const hoja = utils.json_to_sheet(programasData);
    const libro = utils.book_new();
    utils.book_append_sheet(libro, hoja, "Programas");

    writeFile(libro, `${webApp.podcast.info.nombre}_programas.xlsx`);
  }

  function normalizarNombreArchivo(nombre: string): string {
    let normalizado = nombre.toLowerCase();
    normalizado = normalizado.normalize("NFD").replace(/[\u0300-\u036f]/g, "");
    normalizado = normalizado.replace(/[^\w\s]/g, "_");
    normalizado = normalizado.replace(/\s+/g, "_");
    normalizado = normalizado.trim();
    normalizado = normalizado.substring(0, 60); // Máximo 60 caracteres
    if (!normalizado.endsWith(".mp3")) {
      normalizado += ".mp3";
    }
    return normalizado;
  }

  function gestionarRelacionDescargas(evento: Event) {
    const elemento = evento.target as HTMLInputElement;
    const identificador = elemento.getAttribute("data-programa-indice");
    let nombre = elemento.getAttribute("data-programa-nombre");
    if (nombre == "") {
      nombre = identificador;
    }
    const descarga = elemento.getAttribute("data-programa-descarga");
    let infoDescarga: InfoDescarga = {
      identificador: identificador,
      nombre: nombre,
      archivo: normalizarNombreArchivo(nombre),
      descarga: descarga,
      progreso: "Preparando descarga, por favor, espera...",
      finalizado: false,
    };
    if (elemento.checked) {
      webApp.descargas[identificador] = infoDescarga;
    } else {
      delete webApp.descargas[identificador];
    }

    webApp.numeroDescargas = Object.keys(webApp.descargas).length;
  }
</script>

<button id="volver" class="button" onclick={volverBuscar}
  >↩️ Volver a buscar</button
>
<div id="contenedor-analisis">
  {#if webApp.podcast.error_critico != null}
    <div id="error-critico-seccion">
      <div id="error-critico-mensaje" class="notification is-danger">
        <strong>Error crítico:</strong>
        {webApp.podcast.error_critico}.
      </div>
    </div>
  {/if}
  {#if webApp.podcast.errores.length > 0}
    <div id="errores-seccion" class="notification is-warning">
      <strong>Se han encontrado los siguientes errores:</strong>
      <ul id="errores-lista">
        {#each webApp.podcast.errores as error}
          <li>{error}</li>
        {/each}
      </ul>
    </div>
  {/if}

  <div class="columns is-desktop">
    {#if webApp.podcast.info.imagen != null}
      <div class="column">
        <img id="podcast-img" src={webApp.podcast.info.imagen} alt="Imagen" />
      </div>
    {/if}
    {#if webApp.podcast.info.nombre != null && webApp.podcast.info.descripcion != null}
      <div class="column">
        <h5 class="title is-5 has-text-centered">
          {@html webApp.podcast.info.nombre}
          <hr />
          {@html webApp.podcast.info.descripcion}
        </h5>
        <hr />
        <h5 class="title is-5 has-text-centered">
          Programas disponibles: {webApp.podcast.programas.length}
          <br />
          <button onclick={descargarListado}>📥 Descargar listado</button>
        </h5>
      </div>
    {/if}
  </div>

  <form id="formDescargas">
    <div class="section">
      {#each webApp.podcast.programas as programa, indice}
        <div class="box">
          <h3 class="is-5 has-text-weight-medium enlace-programa">
            <a href={programa.url} target="_blank"
              >{programa.titulo ?? `Programa-${indice}`}</a
            >
          </h3>
          <textarea readonly class="textarea"
            >{programa.descripcion ?? ""}</textarea
          >
          {#if programa.descarga != null}
            <div class="checklist-container">
              <label class="checkbox">
                <input
                  onchange={gestionarRelacionDescargas}
                  type="checkbox"
                  name="descargas"
                  data-programa-nombre={programa.titulo ?? ""}
                  data-programa-descarga={programa.descarga}
                  data-programa-indice="Programa-{indice}"
                  value={programa.descarga}
                />
                Descargar programa
              </label>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </form>
</div>

<style>
  #volver {
    margin: 20px auto 50px auto;
    display: block;
  }
  .notification {
    margin-top: 30px;
  }
  #podcast-img {
    display: block;
    margin: 0 auto;
  }
  textarea {
    min-height: 100px;
    max-height: 200px;
    resize: none;
  }
  .checklist-container {
    margin: 30px 0 10px 0;
  }
  .has-text-centered {
    text-align: center;
  }
  .enlace-programa {
    margin: 10px 0 20px 0;
  }
</style>

<script lang="ts">
  import type { WebApp } from "../modelos";

  let ivooxUrl = $state("");
  let deshabilitarForm = $state(false);

  type Props = {
    webApp: WebApp;
  };
  let { webApp = $bindable() }: Props = $props();

  function esUrlVerificada(): boolean {
    // https://www.ivoox.com/podcast-rock-entre-amigos_sq_f1579217_1.html
    if (!ivooxUrl.includes("_sq_")) {
      alert("la url no parece válida, no se ha encontrado _sq_");
      return false;
    }

    if (!ivooxUrl.endsWith("_1.html")) {
      alert("la url no parece válida, no termina en 1.html");
      return false;
    }

    return true;
  }

  async function enviarIvooxUrl(evento: Event) {
    evento.preventDefault();
    webApp.podcast = null;
    webApp.descargas = {};
    webApp.numeroDescargas = 0;
    deshabilitarForm = true;
    if (!esUrlVerificada()) {
      deshabilitarForm = false;
      return;
    }

    let solicitud = {
      url: ivooxUrl,
    };

    let endpoint = "/api/podcast?" + new URLSearchParams(solicitud).toString();
    let f = await fetch(endpoint);
    let r = await f.json();
    webApp.podcast = r;
    deshabilitarForm = false;
  }
</script>

<div id="contenedor">
  <div class="container is-fluid">
    <div class="notification">
      <p class="title">Ivoox Podcast a MP3</p>
      <p class="subtitle">
        Descarga de manera masiva los programas de tu podcast favorito
      </p>
      <form onsubmit={enviarIvooxUrl}>
        <div class="field">
          <label class="label" for="ivoox-url">
            URL principal del podcast:
          </label>
          <div class="control">
            <input
              class="input"
              type="text"
              placeholder="Ivoox URL"
              name="ivoox-url"
              required
              bind:value={ivooxUrl}
            />
          </div>
        </div>
        <input
          type="submit"
          class="button"
          value="Buscar programas"
          disabled={deshabilitarForm}
        />
      </form>
      {#if deshabilitarForm}
        <div id="cargando-contenedor">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 200"
            ><radialGradient
              id="a11"
              cx=".66"
              fx=".66"
              cy=".3125"
              fy=".3125"
              gradientTransform="scale(1.5)"
              ><stop offset="0" stop-color="#008080"></stop><stop
                offset=".3"
                stop-color="#008080"
                stop-opacity=".9"
              ></stop><stop offset=".6" stop-color="#008080" stop-opacity=".6"
              ></stop><stop offset=".8" stop-color="#008080" stop-opacity=".3"
              ></stop><stop offset="1" stop-color="#008080" stop-opacity="0"
              ></stop></radialGradient
            ><circle
              transform-origin="center"
              fill="none"
              stroke="url(#a11)"
              stroke-width="20"
              stroke-linecap="round"
              stroke-dasharray="200 1000"
              stroke-dashoffset="0"
              cx="100"
              cy="100"
              r="70"
              ><animateTransform
                type="rotate"
                attributeName="transform"
                calcMode="spline"
                dur="2"
                values="360;0"
                keyTimes="0;1"
                keySplines="0 0 1 1"
                repeatCount="indefinite"
              ></animateTransform></circle
            ><circle
              transform-origin="center"
              fill="none"
              opacity=".2"
              stroke="#FFFFFF"
              stroke-width="20"
              stroke-linecap="round"
              cx="100"
              cy="100"
              r="70"
            ></circle></svg
          >
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  #contenedor {
    margin-top: 60px;
  }
  #cargando-contenedor {
    display: block;
    margin: 0 auto;
    width: 60px;
  }
</style>

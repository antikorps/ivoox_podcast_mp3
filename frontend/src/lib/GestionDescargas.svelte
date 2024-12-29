<script lang="ts">
  import { onMount } from "svelte";
  import type { InfoDescarga, WebApp } from "../modelos";

  type Props = {
    webApp: WebApp;
  };
  let { webApp = $bindable() }: Props = $props();

  let socket: WebSocket;
  onMount(() => {
    const url = new URL(document.URL);
    const websocket_endpoint = `ws://${url.hostname}:${url.port}/api/descargar`;
    socket = new WebSocket(websocket_endpoint);
    socket.addEventListener("open", () => {
      let coleccion_descargas = [];
      for (const [_, v] of Object.entries(webApp.descargas)) {
        coleccion_descargas.push(v);
      }
      const websocketData = JSON.stringify(coleccion_descargas);
      socket.send(websocketData);
    });
    socket.onmessage = function (event) {
      let respuesta = event.data;
      try {
        let infoDescarga: InfoDescarga = JSON.parse(respuesta);
        webApp.descargas[infoDescarga.identificador] = infoDescarga;
      } catch (error) {
        console.log("error inessperado", error);
      }
    };
  });

  function descargasFinalizadas(): boolean {
    for (const [_, v] of Object.entries(webApp.descargas)) {
      if (v.finalizado == false) {
        return false;
      }
    }
    return true;
  }

  function volverBuscar() {
    if (!descargasFinalizadas()) {
      const confirmacion = confirm(
        '¡Atención! Si hay archivos pendientes de finalización no se va a cancelar su descargar. Si realmente quieres volver atrás pulsa "Aceptar"',
      );
      if (!confirmacion) {
        return;
      }
    }
    webApp.descargando = false;
    webApp.descargas = {};
    webApp.numeroDescargas = 0;
    webApp.podcast = null;
  }
</script>

<button id="volver" class="button" onclick={volverBuscar}
  >↩️ Volver a buscar</button
>

{#each Object.entries(webApp.descargas) as [clave, descarga]}
  <section class="section">
    <div class="highlight-box">
      <h1 class="title has-text-white has-text-centered">{descarga.nombre}</h1>
      <p class="content">{descarga.progreso}</p>
    </div>
  </section>
{/each}

<style>
  #volver {
    margin: 20px auto 50px auto;
    display: block;
  }

  .highlight-box {
    background-color: #008080;
    color: white;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    border-radius: 10px;
    padding: 20px;
  }
  .highlight-box h1 {
    font-size: 2rem;
    margin-bottom: 1rem;
    font-weight: bold;
  }
  .highlight-box p {
    font-size: 1.125rem;
    line-height: 1.5;
    margin-bottom: 1.5rem;
    color: #f4f4f9; /* Color de texto claro */
  }
</style>

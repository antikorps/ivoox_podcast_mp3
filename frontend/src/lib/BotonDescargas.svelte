<script lang="ts">
  import type { WebApp } from "../modelos";

  type Props = {
    webApp: WebApp;
  };
  let { webApp = $bindable() }: Props = $props();

  function comenzarDescargas() {
    webApp.descargando = true;
  }
</script>

<div id="boton-descargas" class="is-fab has-content-centered">
  <button class="button has-shadow is-rounded" onclick={comenzarDescargas}>
    <span>💾 Descargar {webApp.numeroDescargas}</span>
  </button>
</div>

<style>
  #boton-descargas {
    position: fixed;
    bottom: 40px;
    left: 50%;
    transform: translate(-50%, -50%);
  }
</style>

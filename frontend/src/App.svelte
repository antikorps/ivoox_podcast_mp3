<script lang="ts">
  import "./assets/bulma.css";
  import BotonDescargas from "./lib/BotonDescargas.svelte";
  import FormIvooxUrl from "./lib/FormIvooxUrl.svelte";
  import GestionDescargas from "./lib/GestionDescargas.svelte";
  import VistaAnalisis from "./lib/VistaAnalisis.svelte";
  import type { WebApp } from "./modelos";
  let wepAppInicial: WebApp = {
    podcast: null,
    descargas: {},
    descargando: false,
    numeroDescargas: 0,
  };

  type Props = {
    webApp: WebApp;
  };
  let { webApp = $bindable(wepAppInicial) }: Props = $props();
</script>

<div class="container">
  {#if webApp.podcast == null && !webApp.descargando}
    <FormIvooxUrl bind:webApp></FormIvooxUrl>
  {/if}

  {#if webApp.podcast != null && !webApp.descargando}
    <VistaAnalisis bind:webApp></VistaAnalisis>
  {/if}

  {#if webApp.numeroDescargas > 0 && !webApp.descargando}
    <BotonDescargas bind:webApp></BotonDescargas>
  {/if}

  {#if webApp.descargando}
    <GestionDescargas bind:webApp></GestionDescargas>
  {/if}
</div>

<style>
  .container {
    padding: 60px;
  }
</style>

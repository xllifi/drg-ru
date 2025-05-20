<script lang="ts">
  import StatusFeed, { createNotification } from "$lib/components/NotificationsFeed.svelte";
  import { path } from "@tauri-apps/api";
  import { Channel, invoke } from "@tauri-apps/api/core";
  import { _ } from "svelte-i18n";

  let game_path = $state("");
  let paks_path = $derived(game_path.replace(/[/\\]*$/gm, "") + "/FSD/Content/Paks");
  let percent = $state(0)

  let isError: boolean = $state(false)
  let error: string = $state("")

  type ProgressEvent = {
    current: number;
    total: number;
  };
  
  const onEvent = new Channel<ProgressEvent>();
  onEvent.onmessage = (message) => {
    percent = Math.min(Math.max(message.current / message.total * 100, 0), 100)
    console.log(`${message.current}/${message.total} (${percent})`);
  };

  async function unpack(event: Event) {
    event.preventDefault();

    invoke("unpack", {
      inPakPath: `${paks_path}/FSD-WindowsNoEditor.pak`,
      outDirPath: `${paks_path}/temp`,
      onEvent,
    });
  }
  
  async function insert(event: Event) {
    event.preventDefault();

    invoke("insert", {
      inPakPath: `${paks_path}/FSD-WindowsNoEditor.pak`,
      outDirPath: `${paks_path}/temp`,
      onEvent,
    });
  }
  
  async function repack(event: Event) {
    event.preventDefault();

    invoke("unpack", {
      inPakPath: `${paks_path}/FSD-WindowsNoEditor.pak`,
      outDirPath: `${paks_path}/temp`,
      onEvent,
    });
  }

  function onunhandledrejection(e: any) {
    console.log(e)
    console.log(e.reason)
    createNotification('error-unknown', e.reason);
  }

  $inspect(console.log(error))
</script>

<svelte:window {onunhandledrejection} />

<main class="container">
  <p>{paks_path}</p>
  {#if isError}
    <span class="error">error: {error}</span>
  {/if}
  <button onclick={unpack}>Download</button>
  
  <!-- Temporary divider -->
  <p> </p>

  <input placeholder="Game path" bind:value={game_path} />
  <div class="row">
    <button onclick={unpack}>Unpack</button>
    <button >Insert</button>
    <button >Repack</button>
  </div>
  
  <span
    style="
    --percent: {percent}%;
    --percentRaw: {percent};
    "
    class="progressbar"
  ></span>
  <StatusFeed />
</main>

<style lang="scss">
  @use "$lib/styles/vars.scss" as *;

  * {
    font-family: Inter, Roboto, Arial, sans-serif;
  }
  :root {
    font-size: 16px;
    line-height: 20px;
    font-weight: 400;
    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  main {
    height: 100%;
    width: 100%;
    position: relative;
    display: flex;
    flex-direction: column;

    .row {
      display: flex;
      * {
        flex: 1 1 0;
      }
    }

    .progressbar {
      width: 100%;
      height: 32px;
      background: linear-gradient(to right, red var(--percent), black var(--percent));
    }
  }
</style>

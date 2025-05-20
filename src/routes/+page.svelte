<script lang="ts">
  import StatusFeed, {
    createNotification,
  } from "$lib/components/NotificationsFeed.svelte";
  import { Channel, invoke } from "@tauri-apps/api/core";
  import { _ } from "svelte-i18n";

  let game_path = $state("");
  let paks_path = $derived(
    game_path.replace(/[/\\]*$/gm, "").replaceAll("\\", "/") +
      "/FSD/Content/Paks"
  );
  let percent = $state(0);

  // TODO: make it so multiple same tasks can't be ran
  let tasks = $state([]);

  type ProgressEvent = {
    current: number;
    total: number;
  };

  function getOnEvent(): Channel<ProgressEvent> {
    const onEvent = new Channel<ProgressEvent>();
    onEvent.onmessage = (message) => {
      percent = Math.min(
        Math.max((message.current / message.total) * 100, 0),
        100
      );
      console.log(`${message.current}/${message.total} (${percent})`);
    };
    return onEvent
  }

  function clearPercent() {
    percent = 100
    setTimeout(() => {
      percent = 0
    }, 500);
  }

  function checkPath(): boolean {
    if (game_path == "") {
      createNotification("error-nopath");
      return false;
    }
    return true;
  }

  async function download(e: Event) {
    e.preventDefault();
    if (!checkPath()) return;

    invoke("download", {
      archiveUrl: `https://github.com/xllifi/drg-ru/raw/refs/heads/rust/mod_archive/mod_archive.zip`,
      outDirPath: `${paks_path}/temp/mod`,
      onEvent: getOnEvent(), // TODO: Does nothing currently, blocked by download command not reporting progress
    }).finally(clearPercent);
  }

  async function unpack(e: Event) {
    e.preventDefault();
    if (!checkPath()) return;

    invoke("unpack", {
      inPakPath: `${paks_path}/FSD-WindowsNoEditor.pak`,
      outDirPath: `${paks_path}/temp/unpacked`,
      onEvent: getOnEvent(),
    }).finally(clearPercent);
  }

  async function insert(e: Event) {
    e.preventDefault();
    if (!checkPath()) return;

    invoke("insert_files", {
      inDirPath: `${paks_path}/temp/mod`,
      outDirPath: `${paks_path}/temp/unpacked`,
      onEvent: getOnEvent(),
    }).finally(clearPercent);
  }

  async function repack(e: Event) {
    e.preventDefault();
    if (!checkPath()) return;

    invoke("repack", {
      inDirPath: `${paks_path}/temp/unpacked`,
      outPakPath: `${paks_path}/FSD-WindowsNoEditor.pak`,
      onEvent: getOnEvent(),
    }).finally(clearPercent);
  }

  // TODO: handle `AppError`s properly
  function onunhandledrejection(e: any) {
    createNotification("error-unknown", e.reason);
  }

  $effect(() => {
    console.log(game_path);
  });
</script>

<svelte:window {onunhandledrejection} />

<main class="container">
  <p>{paks_path}</p>

  <input placeholder="Game path" bind:value={game_path} />
  <div class="row">
    <button onclick={download}>Download</button>
    <button onclick={unpack}>Unpack</button>
    <button onclick={insert}>Insert</button>
    <button onclick={repack}>Repack</button>
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

  input {
    background-color: $clr-bgl;
    border: none;
    outline: solid 2px $clr-bor;
    outline-offset: -2px;
    padding: 4px;
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
      background: linear-gradient(
        to right,
        red var(--percent),
        black var(--percent)
      );
    }
  }
</style>

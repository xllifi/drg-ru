<script lang="ts">
  import { createNotification } from "$lib/components/NotificationsFeed.svelte";
  import { createProgress, removeProgress, type HexColor, type Progress } from "$lib/components/ProgressBar.svelte";
  import { Channel, invoke } from "@tauri-apps/api/core";
  import { _ } from "svelte-i18n";

  let game_path = $state("");
  let paks_path = $derived(
    game_path.replace(/[/\\]*$/gm, "").replaceAll("\\", "/") +
      "/FSD/Content/Paks"
  );

  // TODO: make it so multiple same tasks can't be ran
  let tasks: Task[] = $state([]);

  type ProgressEvent = 
  | {
    event: 'progress',
    data: {
      current: number;
      total: number;
      task: Task;
    }
  }
  | {
    event: 'finished',
    data: {
      task: Task;
    }
  };
  type Task = "downloadMod" | "unzipMod" | "unpackGame" | "insertFiles" | "repackGame";

  const task_to_color: {[key in Task]: HexColor} = {
    "downloadMod": "#54fff0",
    "unzipMod": "#5456ff",
    "unpackGame": "#ff8454",
    "insertFiles": "#ba54ff",
    "repackGame": "#ff5555",
  }
  
  const task_to_name: {[key in Task]: string} = {
    "downloadMod": "download_mod",
    "unzipMod": "unzip_mod",
    "unpackGame": "unpack_game",
    "insertFiles": "insert_files",
    "repackGame": "repack_game",
  }

  function getOnEvent(): Channel<ProgressEvent> {
    const onEvent = new Channel<ProgressEvent>();
    let progress: Progress | null = $state(null);
    let progressId: number | null = null;
    onEvent.onmessage = (message) => {
      console.log(message)
      console.log(message.event == "progress")
      if (message.event == 'progress') {
        if (!progress) {
          progress = {
            current: message.data.current,
            total: message.data.total,
            color: task_to_color[message.data.task],
            name: $_(`progressbar.tasks.${task_to_name[message.data.task]}`),
          }
          progressId = createProgress(progress)
        } else {
          progress.current = message.data.current
          progress.total = message.data.total
          console.log($state.snapshot(progress))
        }
        console.log(`${message.data.current}/${message.data.total}`);
      }
      if (message.event == 'finished') {
        if (progressId != null) removeProgress(progressId)
      }
    };
    return onEvent
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

    const onEvent = getOnEvent();
    invoke("download", {
      archiveUrl: `https://github.com/xllifi/drg-ru/raw/refs/heads/rust/mod_archive/mod_archive.zip`,
      outDirPath: `${paks_path}/temp/mod`,
      onEvent,
    }).catch(appErrorHandler);
  }

  async function unpack(e: Event) {
    e.preventDefault();
    if (!checkPath()) return;

    const onEvent = getOnEvent();
    invoke("unpack", {
      inPakPath: `${paks_path}/FSD-WindowsNoEditor.pak`,
      outDirPath: `${paks_path}/temp/unpacked`,
      onEvent,
    }).catch(appErrorHandler);
  }

  async function insert(e: Event) {
    e.preventDefault();
    if (!checkPath()) return;

    const onEvent = getOnEvent();
    invoke("insert_files", {
      inDirPath: `${paks_path}/temp/mod`,
      outDirPath: `${paks_path}/temp/unpacked`,
      onEvent,
    }).catch(appErrorHandler);
  }

  async function repack(e: Event) {
    e.preventDefault();
    if (!checkPath()) return;

    const onEvent = getOnEvent();
    invoke("repack", {
      inDirPath: `${paks_path}/temp/unpacked`,
      outPakPath: `${paks_path}/FSD-WindowsNoEditor.pak`,
      onEvent,
    }).catch(appErrorHandler);
  }

  type Cause = 'unknown' | 'downloadNotZip' | 'channelSendFailed' | 'fsError'

  class AppError extends Error {
    cause: Cause;

    constructor(message: string, cause: Cause) {
      super(message)
      this.cause = cause
    }
  }

  function appErrorHandler(e: AppError) {
    switch (e.cause) {
      case "unknown": {
        createNotification('error-unknown', e.message)
        break
      }
      case "downloadNotZip": {
        createNotification('error-dlnotzip', e.message)
        break
      }
      case "channelSendFailed": {
        createNotification('error-channelsendfailed', e.message)
        break
      }
      case "fsError": {
        createNotification('error-fs', e.message)
        break
      }
    }
  }
</script>

<svelte:window {onunhandledrejection} />

<main class="container">
  <label class="gamepath">
    <span class="preview">{game_path ? paks_path : "Введите путь до игры"}</span>
    <span>{$_('main.game_path')}:</span>
    <input placeholder="C:\Program Files (x86)\Steam\steamapps\common\Deep Rock Galactic" bind:value={game_path} />
  </label>
  <div class="row">
    <button onclick={download}>1. {$_('main.buttons.download')}</button>
    <button onclick={unpack}  >2. {$_('main.buttons.unpack')}</button>
    <button onclick={insert}  >3. {$_('main.buttons.insert')}</button>
    <button onclick={repack}  >4. {$_('main.buttons.repack')}</button>
  </div>
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
    padding: 8px;
    border-radius: 8px;
  }

  main {
    height: calc(100% - 16px);
    width: calc(100% - 16px);
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px;

    .gamepath {
      position: relative;
      display: flex;
      align-items: center;
      gap: 8px;
      margin-top: 12px;
      input {
        flex-grow: 1;
      }
      .preview {
        position: absolute;
        opacity: 0.2;
        top: 0;
        font-size: 12px;
        text-align: center;
        width: 100%;
        transform: translateY(-100%);
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
      }
    }

    .row {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;

      button {
        flex-grow: 1;
        padding: 8px;
        // min-width: 40%;
      }
    }
  }
</style>

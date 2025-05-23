<script lang="ts" module>
  import { ChevronDown, ChevronUp, GripHorizontal } from "lucide-svelte";
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";

  export type HexColor = `#${string}`;
  export type Progress = {
    current: number;
    total: number;
    color: HexColor;
    name: string;
    id?: number;
  };
  let progresses: Progress[] = $state([]);

  function getPercent(current: number, total: number): number {
    return Math.min(Math.max((current / total) * 100, 0), 100);
  }

  export function createProgress(progress: Progress): number {
    console.log(`Requested progress creation with object:`);
    console.log($state.snapshot(progress));
    let minId = -1;
    if (progresses.map((x) => x.id).includes(0)) {
      minId = Math.max(...progresses.map((x) => x.id!));
    }
    const id = minId + 1;
    console.log(`minId: ${minId}, assigning new progress id ${id}`);

    progress.id = id;
    progresses.push(progress);
    return id;
  }
  export function removeProgress(id: number) {
    setTimeout(() => {
      const index = progresses.findIndex((x) => x.id == id);
      if (!index && index != 0) {
        throw `Couldn't find progress with id ${id}`;
      }
      progresses.splice(index, 1);
    }, 2000);
  }

  let isExtended = $state(false);
</script>

<main>
  {#if progresses.length > 0}
    <button
      transition:fly={{ y: 100 }}
      class="progressbars"
      class:extend={isExtended}
      onclick={() => {
        isExtended = !isExtended;
      }}
    >
      <span class="toggle_extend">
        {#if isExtended}
          <ChevronDown />
        {:else}
          <ChevronUp />
        {/if}
      </span>
      {#each progresses as progress (progress.id)}
        <div
          in:fly={{ y: 32, duration: 500 }}
          animate:flip={{ duration: 300 }}
          style="
          --percent: {getPercent(progress.current, progress.total)}%;
          --percentRaw: {getPercent(progress.current, progress.total)};
          --color: {progress.color};
        "
          class="progressbar"
        >
          <span class="label"
            >{progress.name} ({getPercent(progress.current, progress.total).toFixed(2)}%)</span
          >
        </div>
      {/each}
    </button>
  {/if}
</main>

<style lang="scss">
  @use "$lib/styles/vars.scss" as *;

  main {
    position: fixed;
    bottom: 0;
    width: 100dvw;

    display: flex;
    flex-direction: column;
    align-items: center;
    pointer-events: none;
  }

  .progressbars {
    outline: none;
    --padding-horizontal: 8px;
    --padding-vertical: 4px;
    width: calc(100dvw - var(--padding-horizontal));
    max-width: 28rem;
    display: flex;
    flex-direction: column;
    padding: var(--padding-vertical) var(--padding-horizontal);
    gap: 6px;

    background-color: $clr-bgl;
    border-radius: 8px 8px 0 0;

    filter: drop-shadow(0 0 8px #0006);

    transition: padding 200ms;
    pointer-events: auto;

    &.extend {
      --padding-vertical: 8px;
      .progressbar {
        height: 32px;
        color: inherit;

        .label {
          opacity: 1;
        }
      }
    }

    * {
      pointer-events: none;
    }

    .toggle_extend {
      height: 8px;
      display: flex;
      justify-content: center;
      align-items: center;

      // outline: dashed 2px red;
      border-radius: 0;

      :global(.lucide) {
        width: 16px;
      }
    }

    .progressbar {
      border-radius: 4px;
      width: 100%;
      height: 4px;
      background: linear-gradient(
        to right,
        var(--color) var(--percent),
        black var(--percent)
      );
      display: flex;
      justify-content: center;
      align-items: center;

      transition: height 200ms;

      .label {
        --after_progress_line: white;
        --befor_progress_line: black;
        width: 100%;
        background: linear-gradient(
          to right,
          var(--befor_progress_line) var(--percent),
          var(--after_progress_line) var(--percent)
        );
        color: transparent;
        background-clip: text;
        font-weight: 600;
        opacity: 0;

        transition: opacity 100ms;
      }
    }
  }
</style>

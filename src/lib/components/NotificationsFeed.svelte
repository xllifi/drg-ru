<script lang="ts" module>
  import { Trash2, Copy } from "lucide-svelte";
  import { _ } from "svelte-i18n";
  import { flip } from "svelte/animate";
  import { fly, slide } from "svelte/transition";

  type Notification = {
    title: string;
    description: string;
    verbose?: string;
    showVerbose?: boolean;
    id: number;
  };

  let notifications: Notification[] = $state([]);

  export function createNotification(messageId: string, verbose?: string) {
    let minId = -1;
    if (notifications.map((x) => x.id).includes(0)) {
      minId = Math.max(...notifications.map((x) => x.id));
    }
    console.log(`minId: ${minId}, assigning new notification id ${minId + 1}`);

    notifications = [
      ...notifications,
      {
        title: `noti.messages.${messageId}.title`,
        description: `noti.messages.${messageId}.description`,
        verbose,
        id: minId + 1,
      },
    ];
  }

  function close(e: Event) {
    const el = e.target as HTMLButtonElement;
    const id = parseInt(el.id.replace("close", ""));
    removeNotification(id);
  }

  function copy(notification: Notification) {
    navigator.clipboard.writeText(
      `${notification.title}\n${notification.description}` +
        (notification.verbose ? `\n\nVerbose:\n${notification.verbose}` : "")
    );
  }

  function removeNotification(id: number) {
    const index = notifications.findIndex((x) => x.id == id);
    if (!index && index != 0) {
      throw `Couldn't find notification id ${id}`;
    }
    notifications.splice(index, 1);
    // So Svelte updates it
    notifications = notifications;
  }
</script>

<div class="statusfeed">
  <div class="box">
    {#each notifications as n (n.id)}
      <div
        class="notification"
        in:fly={{ x: 100, delay: 100, duration: 200 }}
        out:fly={{ x: 100, duration: 100 }}
        animate:flip={{ duration: 200 }}
      >
        <div class="title">
          <h2>{$_(n.title)}</h2>
          <span class="empty"></span>
          <button onclick={() => copy(n)}><Copy /></button>
          <button onclick={close} id="close{n.id}"><Trash2 /></button>
        </div>
        <p>{$_(n.description)}</p>
        {#if n.verbose}
          <button
            class="toggle_verbose"
            onclick={() => (n.showVerbose = !n.showVerbose)}
            >{$_("noti.base.verbose")}</button
          >
          {#if n.showVerbose}
            <div class="verbose" transition:slide>
              <span>{n.verbose}</span>
            </div>
          {/if}
        {/if}
      </div>
    {/each}
  </div>
</div>

<style lang="scss">
  @use "$lib/styles/vars.scss" as *;

  .statusfeed {
    position: fixed;
    height: calc(100% - 16px);
    width: 20rem;
    right: 0;
    bottom: 0;
    overflow-y: scroll;
    overflow-x: hidden;

    display: flex;
    flex-direction: column-reverse;
    padding: 8px 0;

    filter: drop-shadow(0 4px 4px #0006);

    pointer-events: none;

    // debug style
    // outline: dashed 2px red;
    // outline-offset: -2px;

    &::-webkit-scrollbar {
      width: 0;
      height: 0;
    }

    div.box {
      display: flex;
      flex-direction: column;
      min-height: min-content;
      gap: 8px;
      padding-right: 8px;

      pointer-events: auto;

      div.notification {
        background-color: $clr-bgl;
        border-radius: 8px;
        padding: 8px;

        > *:not(:first-child) {
          margin-top: 4px;
        }

        .title {
          min-height: 32px;
          display: flex;
          justify-content: center;
          align-items: center;

          gap: 4px;
          margin: -8px;
          margin-bottom: 4px;
          padding: 0 8px;

          background-color: $clr-bgls;
          border-radius: 8px 8px 0 0;

          span.empty {
            flex-grow: 1;
          }

          h2 {
            padding: 8px 0;
            margin: 0;
            font-size: 20px;
          }
          button {
            background-color: transparent;
            display: flex;
            justify-content: center;
            align-items: center;
            border-radius: 4px;
            outline: none;
            opacity: 0.4;
            padding: 0;

            transition: opacity 200ms;

            &:hover {
              opacity: 1;
            }

            :global(.lucide) {
              pointer-events: none;
              height: 16px;
            }
          }
        }
        p {
          margin: 0;
        }
        .verbose {
          background-color: $clr-bg;
          padding: 4px 8px;
          border-radius: 4px;
          span {
            font-family: monospace;
          }
        }
      }
    }
  }
</style>

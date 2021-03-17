<script lang="ts">
  import type { TransitionConfig } from 'svelte/transition';
  import { quadOut } from 'svelte/easing';
  import { createEventDispatcher } from 'svelte';
  import Overlay from '../Overlay.svelte';
  export let visible: boolean;

  function modal(_: HTMLElement, opts: TransitionConfig) {
    return {
      ...opts,
      css: (t: number, _: number) => {
        return `
          transform: scale(${t})
        `;
      },
    };
  }
</script>

{#if visible}
  <Overlay on:close>
    <div class="modal" transition:modal={{ duration: 200, easing: quadOut }}>
      <slot />
    </div>
  </Overlay>
{/if}

<style>
  .modal {
    background: var(--background);
    border-radius: var(--border-radius);
    width: 100%;
    padding: var(--standard-padding);
    max-width: 500px;
    overflow-y: auto;
    user-select: none;
  }
</style>

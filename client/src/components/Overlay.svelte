<script lang="ts">
  import { fade } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let zIndex: number = 100;

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      dispatch('close');
    }
  }
</script>

<div
  class="overlay"
  style="z-index: {zIndex};"
  on:click|self={() => dispatch('close')}
  transition:fade={{ duration: 100 }}>
  <slot />
</div>

<svelte:window on:keydown={handleKeydown} />

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    padding: 1em;
    background: rgba(0, 0, 0, 0.8);
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
</style>

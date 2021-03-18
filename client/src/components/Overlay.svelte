<script lang="ts" context="module">
  let overlayStack = 0;
</script>

<script lang="ts">
  import { fade } from 'svelte/transition';
  import { createEventDispatcher, onMount } from 'svelte';
  const dispatch = createEventDispatcher();

  let id: number;
  let zIndex = 100;

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && id === overlayStack) {
      dispatch('close');
    }
  }

  onMount(() => {
    overlayStack++;
    id = overlayStack;
    zIndex = id * 100;

    return () => {
      overlayStack--;
    };
  });
</script>

<div
  class="overlay"
  style="z-index: {zIndex};"
  transition:fade={{ duration: 100 }}>
  <div class="backdrop" on:click={() => dispatch('close')} />
  <slot />
</div>

<svelte:window on:keydown={handleKeydown} />

<style>
  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    padding: 1em;
    flex-direction: column;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .backdrop {
    position: absolute;
    z-index: -1;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.8);
  }
</style>

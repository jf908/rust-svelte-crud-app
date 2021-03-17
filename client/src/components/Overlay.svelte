<script lang="ts" context="module">
  let overlayStack = 0;
</script>

<script lang="ts">
  import { fade } from 'svelte/transition';
  import { createEventDispatcher, onMount } from 'svelte';
  import { writable } from 'svelte/store';
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

<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import type { Tag } from '../lib/api';
  import { tags } from '../store';
  import InlineTag from './InlineTag.svelte';

  export let exclude: number[];
  export let parent: Node | undefined;

  const dispatch = createEventDispatcher();
  let el: HTMLElement;

  let search = '';

  let filteredTags: Tag[];
  $: filteredTags = $tags
    ? $tags.filter((x) => !exclude.includes(x.id) && x.name.startsWith(search))
    : [];

  function onSelect(id: number) {
    dispatch('select', id);
  }

  function onClick(e: MouseEvent) {
    if (e.target !== parent && !el.contains(e.target as Node)) {
      dispatch('close');
    }
  }

  onMount(() => {
    window.addEventListener('click', onClick);

    return () => {
      window.removeEventListener('click', onClick);
    };
  });
</script>

<div
  class="tag-selector"
  bind:this={el}
  transition:fly={{ duration: 200, y: -20 }}>
  <div class="search">
    <input type="text" placeholder="Search" bind:value={search} />
  </div>
  {#each filteredTags as tag}
    <InlineTag {tag} on:click={() => onSelect(tag.id)} />
  {:else}No tags{/each}
</div>

<style>
  .tag-selector {
    position: absolute;
    background: var(--background);
    border-radius: var(--border-radius);
    padding: var(--standard-padding);
    width: 300px;
    box-shadow: var(--hover-shadow);
    z-index: 10;
  }

  .tag-selector :global(.tag) {
    cursor: pointer;
  }

  .search input {
    width: 100%;
    margin-bottom: 1em;
  }
</style>

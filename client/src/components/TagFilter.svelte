<script lang="ts">
  import { tags } from '../store';
  import InlineTag from './InlineTag.svelte';

  export let selected: Set<number> = new Set();

  function toggleTag(id: number) {
    if (selected.has(id)) {
      selected.delete(id);
    } else {
      selected.add(id);
    }
    selected = selected;
  }
</script>

<div class="tag-filter">
  {#if $tags}
    {#each $tags as tag}
      <InlineTag
        {tag}
        selected={selected.has(tag.id)}
        on:click={() => toggleTag(tag.id)} />
    {/each}
  {/if}
</div>

<style>
  .tag-filter {
    display: inline;
  }

  .tag-filter :global(.tag) {
    cursor: pointer;
  }
</style>

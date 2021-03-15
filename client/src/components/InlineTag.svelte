<script lang="ts">
  import type { Tag } from '../lib/api';

  export let tag: Tag;
  export let selected = true;
</script>

<div class="tag" title="ID:{tag.id}" class:selected on:click>{tag.name}</div>

<style>
  .tag {
    font-size: var(--font-size-small);
    padding: var(--small-padding);
    display: inline-block;
    border-radius: 2em;
    background: rgba(0, 0, 0, 0.1);
    margin-right: 0.5em;
    cursor: pointer;
    user-select: none;
    transition: opacity 0.2s;

    opacity: 0.5;
  }

  .tag.selected {
    opacity: 1;
  }
</style>

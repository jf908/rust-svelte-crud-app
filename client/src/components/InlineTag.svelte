<script lang="ts">
  import type { Tag } from '../lib/api';

  export let tag: Tag;
  export let selected = true;
</script>

<div class="tag" title="ID:{tag.id}" class:selected on:click>{tag.name}</div>

<style>
  .tag {
    transition: opacity 0.2s;
    opacity: 0.5;
  }

  .tag.selected {
    opacity: 1;
  }
</style>

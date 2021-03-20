<script lang="ts">
  import type { Tag } from '../lib/api';
  import { getBackgroundTagColor, getTextTagColor } from '../lib/color';

  export let tag: Tag;
  export let selected = true;
</script>

<div
  class="tag"
  title="ID:{tag.id}"
  class:selected
  style="background: {getBackgroundTagColor(tag.color)}; color: {getTextTagColor(tag.color)};"
  on:click>
  {tag.name}
</div>

<style>
  .tag {
    transition: opacity 0.2s;
    opacity: 0.5;
  }

  .tag.selected {
    opacity: 1;
  }
</style>

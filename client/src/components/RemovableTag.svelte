<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Tag } from '../lib/api';
  import { tagsMap } from '../store';
  import Icon from './Icon.svelte';

  export let id: number;

  const dispatch = createEventDispatcher();

  function remove() {
    dispatch('remove');
  }
  let tag: Tag;
  $: tag = $tagsMap[id];
</script>

<div class="tag" title="ID:{tag.id}">
  {tag.name}
  <div class="close">
    <Icon on:click={remove} icon="smallClose" />
  </div>
</div>

<style>
  .close {
    display: inline-block;
    cursor: pointer;
    opacity: 0.6;
  }

  .close:hover {
    opacity: 1;
  }
</style>

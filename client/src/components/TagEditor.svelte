<script lang="ts">
  import type { Tag } from '../lib/api';

  import { confirmModal, tags } from '../store';
  import InlineTag from './InlineTag.svelte';

  let tagName = '';

  function createTag() {
    let formatted = tagName.trim();

    if (formatted === '') return;

    tags.addTag(formatted);

    tagName = '';
  }

  function deleteTag(tag: Tag) {
    $confirmModal = {
      body: `Tag ${tag.name}`,
      confirm: () => tags.deleteTag(tag.id),
    };
  }
</script>

<div class="tags">
  <h2>Tag Editor</h2>
  <form on:submit|preventDefault={createTag}>
    <input type="text" bind:value={tagName} placeholder="Create tag" />
    <button on:click={createTag}>+</button>
  </form>
  {#if $tags}
    {#each $tags as tag}
      <div class="row">
        <InlineTag {tag} />
        <button on:click={() => deleteTag(tag)}>Delete</button>
      </div>
    {/each}
  {/if}
</div>

<style>
  h2 {
    margin-bottom: 1em;
  }

  .tags {
    text-align: left;
    margin: auto;
  }

  .row {
    padding: 0.25em;
  }

  .row button {
    font-size: 12px;
  }
</style>

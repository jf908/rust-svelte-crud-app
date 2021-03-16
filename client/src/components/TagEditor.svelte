<script lang="ts">
  import type { Tag } from '../lib/api';

  import { confirmModal, tags } from '../store';
  import Icon from './Icon.svelte';
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
    <button on:click={createTag}> + </button>
  </form>
  {#if $tags}
    {#each $tags as tag}
      <div class="row">
        <InlineTag {tag} />
        <div class="actions">
          <button on:click={() => deleteTag(tag)}>Delete</button>
        </div>
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

  form {
    margin-bottom: 0.5em;
  }

  .row {
    display: flex;
    border-radius: var(--border-radius);
    align-items: baseline;
    padding: 0.1em 0.5em;
  }

  .row .actions {
    margin-left: auto;
    visibility: hidden;
  }

  .row:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .row:hover .actions {
    visibility: visible;
  }

  .row button {
    font-size: 12px;
  }
</style>

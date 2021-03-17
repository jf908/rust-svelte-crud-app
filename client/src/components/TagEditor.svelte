<script lang="ts">
  import type { Tag } from '../lib/api';

  import { confirmModal, tags } from '../store';
  import AsyncTextBox from './AsyncTextBox.svelte';
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

  async function editTag(id: number, name: string) {
    await tags.editTag(id, name);
  }
</script>

<div class="tags">
  <h2>Tag Editor</h2>
  <form on:submit|preventDefault={createTag}>
    <input type="text" bind:value={tagName} placeholder="Create tag" />
    <button on:click={createTag}> + </button>
  </form>
  <div class="rows">
    {#if $tags}
      {#each $tags as tag}
        <div class="row">
          <AsyncTextBox
            asyncChange={(v) => editTag(tag.id, v)}
            class="tag-name"
            bind:value={tag.name} />
          <div class="actions">
            <button on:click={() => deleteTag(tag)}>Delete</button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  h2 {
    margin-bottom: 1em;
  }

  .tags {
    display: flex;
    flex-direction: column;
    text-align: left;
    margin: auto;
    height: 100%;
  }

  form {
    display: flex;
    margin-bottom: 0.5em;
  }

  form input {
    flex: 1;
    margin-right: 0.5em;
  }

  .rows {
    overflow-y: auto;
  }

  .row {
    display: flex;
    border-radius: var(--border-radius);
    align-items: baseline;
    padding: 0.25em 0.5em;
  }

  .row .actions {
    margin-left: auto;
    visibility: hidden;
  }

  .row:hover {
    background: var(--hover-color);
  }

  .row:hover .actions {
    visibility: visible;
  }

  .row button {
    font-size: var(--font-size-small);
  }

  .row :global(.tag-name) {
    font-size: var(--font-size-small);
    border-color: transparent;
  }

  .row:hover :global(.tag-name),
  .row :global(.tag-name:focus) {
    border-color: var(--border-color);
  }
</style>

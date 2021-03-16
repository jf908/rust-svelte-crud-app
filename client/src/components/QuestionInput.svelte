<script lang="ts">
  import { questions, tagEditor } from '../store';

  import AutoHeightTextarea from './AutoHeightTextarea.svelte';
  import TagFilter from './TagFilter.svelte';

  let body: string = '';
  let selectedTags: Set<number>;
  let root: HTMLElement;

  function onKeyDown(e: KeyboardEvent) {
    if (e.code === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      questions.addQuestion(body, Array.from(selectedTags));

      body = '';
    }
  }

  function onMouseDown(e: MouseEvent) {
    if (e.button === 0) {
      let textarea: HTMLTextAreaElement | null = root.querySelector(
        '.input-textarea textarea'
      );
      if (textarea) {
        textarea.focus();
      }
    }
  }
</script>

<div class="input" bind:this={root}>
  <AutoHeightTextarea
    placeholder="Write question..."
    class="input-textarea"
    bind:value={body}
    on:keydown={onKeyDown} />
  <div class="tag-selector" on:mousedown|self={onMouseDown}>
    <TagFilter bind:selected={selectedTags} /><button
      class="edit-button"
      on:click={() => ($tagEditor = true)}>Edit...</button>
  </div>
</div>

<style>
  .input {
    padding: 0;
    text-align: left;
  }

  .input :global(.input-textarea) {
    display: block;
    border: 0;
    width: 100%;
    padding: var(--standard-padding);
  }

  .tag-selector {
    padding: 0 8px 8px;
    /* Cancel out .tag padding-bottom */
    margin-bottom: -6px;
  }

  .edit-button {
    font-size: var(--font-size-small);
    border: 0;
    padding: var(--small-padding);
  }
</style>

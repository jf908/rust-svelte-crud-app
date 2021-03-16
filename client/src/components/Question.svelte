<script lang="ts">
  import AutoHeightTextarea from './AutoHeightTextarea.svelte';
  import { confirmModal, questions, tags, tagsMap } from '../store';
  import Tag from './InlineTag.svelte';
  import RemovableTag from './RemovableTag.svelte';
  import Icon from './Icon.svelte';
  import TagSelector from './TagSelector.svelte';
  import type { Question } from '../lib/api';
  import Date from './Date.svelte';

  export let question: Question;
  let selectingTag = false;

  let addTagEl: Node;
  let prevBody: string | undefined;

  let editing = false;

  function deleteQuestion() {
    $confirmModal = {
      body: `Question: ${question.body}`,
      confirm: () => questions.deleteQuestion(question.id),
    };
  }

  function startEdit() {
    prevBody = question.body;
    editing = true;
  }

  function cancelEdit() {
    if (prevBody !== undefined) {
      question.body = prevBody;
    }
    editing = false;
  }

  function saveEdit() {
    questions.editQuestion(question.id, question.body);
    editing = false;
  }

  async function addTag(tagId: number) {
    await questions.addTag(question.id, tagId);
    question.tags = [...question.tags, tagId];
  }

  async function removeTag(tagId: number) {
    await questions.removeTag(question.id, tagId);
    question.tags = question.tags.filter((x) => x !== tagId);
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.code === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      saveEdit();
    }
  }
</script>

<div class="question" class:editing>
  {#if editing}
    <AutoHeightTextarea
      class="question-edit"
      autofocus
      on:keydown={onKeyDown}
      bind:value={question.body} />
  {:else}
    <div class="body" title="ID:{question.id}">{question.body}</div>
  {/if}
  <div class="tags">
    <div class="no-whitespace">
      {#each question.tags as id}
        <RemovableTag {id} on:remove={() => removeTag(id)} />
      {/each}
      <div
        class="tag tag-add"
        bind:this={addTagEl}
        on:click={() => (selectingTag = !selectingTag)}>
        +
      </div>
    </div>
    {#if selectingTag}
      <TagSelector
        parent={addTagEl}
        exclude={question.tags}
        on:select={({ detail }) => addTag(detail)}
        on:close={() => (selectingTag = false)} />
    {/if}
  </div>
  <div class="actions">
    {#if editing}
      <button on:click={saveEdit}>Save</button>
      <button on:click={cancelEdit}>Cancel</button>
    {:else}
      <Date date={question.created_at} />
      <button on:click={startEdit}>Edit</button>
      <button on:click={deleteQuestion}>Delete</button>
    {/if}
  </div>
</div>

<style>
  .question {
    position: relative;
    /* border: var(--default-border); */
    border-radius: var(--border-radius);
    margin: 1em 0;
    padding: var(--standard-padding);
    /* padding-bottom: 0; */
  }

  .question:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .question :global(.question-edit) {
    background: none;
    display: block;
    width: 100%;
    border: 0;
    border-radius: 0;
    padding: 0;
  }

  button {
    font-size: 12px;
  }

  .no-whitespace {
    font-size: 0;
  }

  .tags {
    /* Cancel out .tag padding-bottom */
    margin-bottom: -6px;
  }

  .tag-add {
    padding: 3px 7px;
    cursor: pointer;
  }

  .question :global(.question-edit),
  .body {
    margin-bottom: 0.5em;
    white-space: pre-wrap;
  }

  .actions {
    position: absolute;
    top: -1em;
    right: 0.5em;
    visibility: hidden;
  }

  .question:hover .actions,
  .question.editing .actions {
    visibility: visible;
  }
</style>

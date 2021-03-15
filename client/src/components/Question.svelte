<script lang="ts">
  import AutoHeightTextarea from './AutoHeightTextarea.svelte';
  import type { Question } from '../lib/api';
  import { confirmModal, questions, tags, tagsMap } from '../store';
  import Tag from './InlineTag.svelte';

  export let question: Question;

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
</script>

<div class="question">
  {#if editing}
    <AutoHeightTextarea bind:value={question.body} />
    <button on:click={saveEdit}>Save</button>
    <button on:click={cancelEdit}>Cancel</button>
  {:else}
    <div class="body">{question.body}</div>
  {/if}
  <div class="tags">
    {#each question.tags as tag}
      <Tag tag={$tagsMap[tag]} />
    {/each}
  </div>
  <button on:click={startEdit}>Edit</button>
  <button on:click={deleteQuestion}>Delete</button>
</div>

<style>
  .question {
    border: var(--default-border);
    border-radius: var(--border-radius);
    margin: 1em 0;
    padding: var(--standard-padding);
  }

  button {
    font-size: 12px;
  }
</style>

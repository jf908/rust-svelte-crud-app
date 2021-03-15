<script lang="ts">
  import Loading from './components/Loading.svelte';
  import ConfirmModal from './components/modal/ConfirmModal.svelte';
  import Question from './components/Question.svelte';
  import QuestionInput from './components/QuestionInput.svelte';
  import TagEditor from './components/TagEditor.svelte';
  import TagEditorOverlay from './components/TagEditorOverlay.svelte';
  import { questions, reconnect } from './store';

  let connection = reconnect();
</script>

<ConfirmModal />

<TagEditorOverlay />
<div class="app">
  <aside>
    <h1>Question Editor</h1>
  </aside>
  <main>
    <div class="container">
      {#await connection}
        <Loading />
        <p>Loading...</p>
      {:then _}
        <QuestionInput />
        {#if $questions}
          {#each $questions as question}
            <Question {question} />
          {/each}
        {/if}
      {:catch error}
        <p>Failed to load</p>
        <button on:click={() => (connection = reconnect())}>Reconnect</button>
      {/await}
    </div>
  </main>
</div>

<style>
  .app {
    display: flex;
    min-height: 100vh;
  }

  aside {
    border-right: var(--default-border);
    width: 250px;
    text-align: center;
  }

  main {
    padding: 1em 0;
    flex: 1;
  }
</style>

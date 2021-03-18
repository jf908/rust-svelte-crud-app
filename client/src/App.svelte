<script lang="ts">
  import Loading from './components/Loading.svelte';
  import ConfirmModal from './components/modal/ConfirmModal.svelte';
  import Question from './components/Question.svelte';
  import QuestionInput from './components/QuestionInput.svelte';
  import TagEditorOverlay from './components/TagEditorOverlay.svelte';
  import { moreQuestions, questions, reconnect } from './store';

  let connection = reconnect();

  let gettingMore = false;

  async function loadMore() {
    gettingMore = true;
    try {
      await questions.getMore();
    } finally {
      gettingMore = false;
    }
  }
</script>

<ConfirmModal />

<TagEditorOverlay />
<div class="app">
  <!-- <aside>
    <h1>Question Editor</h1>
  </aside> -->
  <main>
    <div class="container">
      {#await connection}
        <p class="center">
          <Loading />
          Loading...
        </p>
      {:then _}
        <QuestionInput />
        {#if $questions}
          {#each $questions as question}
            <Question {question} />
          {/each}
          {#if $moreQuestions}
            <div class="center">
              {#if gettingMore}
                <Loading />
              {:else}<button on:click={loadMore}>Load more</button>{/if}
            </div>
          {/if}
        {/if}
      {:catch error}
        <div class="center">
          <p>Failed to load</p>
          <button on:click={() => (connection = reconnect())}>Reconnect</button>
        </div>
      {/await}
    </div>
  </main>
</div>

<style>
  .app {
    display: flex;
    min-height: 100vh;
  }

  /* aside {
    border-right: var(--default-border);
    width: 250px;
    text-align: center;
  } */

  main {
    padding: 1em 0;
    flex: 1;
  }

  .center {
    text-align: center;
  }
</style>

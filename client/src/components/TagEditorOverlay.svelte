<script>
  import { tagEditor } from '../store';

  import Overlay from './Overlay.svelte';

  import TagEditor from './TagEditor.svelte';
</script>

{#if $tagEditor}
  <Overlay on:close={() => ($tagEditor = false)}>
    <div class="back">
      <TagEditor />
    </div>
  </Overlay>
{/if}

<style>
  .back {
    background: var(--background);
    border-radius: var(--border-radius);
    padding: 2em;
    max-width: 400px;
    width: 100%;
    min-height: 100%;
  }
</style>

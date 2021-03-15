<script>
  import { tagEditor } from '../store';

  import Overlay from './Overlay.svelte';

  import TagEditor from './TagEditor.svelte';
</script>

{#if $tagEditor}
  <Overlay zIndex={50} on:close={() => ($tagEditor = false)}>
    <div class="back">
      <TagEditor />
    </div>
  </Overlay>
{/if}

<style>
  .back {
    background: var(--background);
    padding: 2em;
    border-radius: var(--border-radius);
    min-width: 800px;
  }
</style>

<script lang="ts">
  // import Icon from '../Icon.svelte';
  import Modal from './Modal.svelte';
  import { confirmModal } from '../../store';

  function confirm() {
    if ($confirmModal) {
      $confirmModal.confirm();
      $confirmModal = null;
    }
  }

  function cancel() {
    $confirmModal = null;
  }
</script>

<Modal visible={!!$confirmModal} on:close={cancel}>
  <header>
    <h3>Are you sure you want to delete this?</h3>
    <div class="close link" on:click={cancel}>
      <!-- <Icon icon="close" hover="fade" /> -->
      Close
    </div>
  </header>
  <div class="body">{$confirmModal.body}</div>
  <footer>
    <button on:click={cancel}>Cancel</button>
    <button class="danger" on:click={confirm}>Delete</button>
  </footer>
</Modal>

<style>
  header {
    display: flex;
    align-items: baseline;
    margin-bottom: 1em;
  }

  h3 {
    margin: 0;
    margin-bottom: 1em;
  }

  .close {
    cursor: pointer;
    margin-left: auto;
  }

  footer {
    text-align: right;
  }
</style>

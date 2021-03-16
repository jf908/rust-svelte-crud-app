<script lang="ts">
  import icons from '../lib/icons';

  export let icon: keyof typeof icons;
</script>

<!-- prettier-ignore -->
<div class="icon" on:click>{@html icons[icon]}</div>

<style>
  .icon {
    display: inline-flex;
    vertical-align: middle;
  }
</style>

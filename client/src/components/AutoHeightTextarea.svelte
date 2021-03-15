<script>
  import { onMount } from 'svelte';

  let className: string;
  export { className as class };
  let el: HTMLTextAreaElement;
  export let value: string;
  export let placeholder: string | undefined = undefined;

  $: watch(value);
  function watch(_: string) {
    if (el) {
      el.style.height = 'auto';
      el.style.height = el.scrollHeight + 'px';
    }
  }

  onMount(() => {
    watch(value);
  });
</script>

<textarea
  class={className}
  {placeholder}
  bind:this={el}
  bind:value
  on:keydown
  rows="1" />

<style>
  textarea {
    overflow: hidden;
  }
</style>

<script lang="ts">
  export let value: string;
  export let className = '';

  export let asyncChange: (value: string) => Promise<void>;

  export { className as class };

  let temp = value;
  $: temp = value;

  async function onChange() {
    try {
      let v = temp;
      await asyncChange(v);
      value = temp;
    } catch (ex) {
      // No Change
    }
  }
</script>

<input class={className} type="text" bind:value={temp} on:change={onChange} />

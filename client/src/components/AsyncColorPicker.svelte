<script lang="ts">
  import { getBackgroundTagColor } from '../lib/color';

  export let value: number | null;
  export let className = '';

  export let asyncChange: (value: number) => Promise<void>;

  export { className as class };

  let temp: string;
  $: temp = getBackgroundTagColor(value);

  async function onChange() {
    try {
      let v = parseInt(
        temp.substr(1, 2) + temp.substr(3, 2) + temp.substr(5, 2),
        16
      );
      await asyncChange(v);
      value = v;
    } catch (ex) {
      // No Change
    }
  }
</script>

<input class={className} type="color" bind:value={temp} on:change={onChange} />

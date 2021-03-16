<script lang="ts">
  export let date: string;
</script>

<span>{new Date(date).toLocaleDateString()}</span>

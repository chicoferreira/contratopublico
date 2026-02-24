<script lang="ts">
  let { title, description } = $props();
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />

  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />

  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
</svelte:head>

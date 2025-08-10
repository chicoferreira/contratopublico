<script lang="ts">
  import { ArrowLeft } from "@lucide/svelte";

  const { title, children } = $props();
</script>

<svelte:head>
  <title>Contrato Público - {title}</title>
</svelte:head>

<a
  href="/"
  class="text-muted-foreground group-hover:text-muted-foreground/70 group mb-4 flex cursor-pointer items-center gap-2 transition-colors duration-200">
  <ArrowLeft class="h-4 w-4" />
  <span class="font-medium">Voltar à pesquisa</span>
</a>

<div class="prose max-w-none">
  {@render children?.()}
</div>

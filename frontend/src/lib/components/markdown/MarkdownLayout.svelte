<script lang="ts">
  import { ArrowLeft } from "@lucide/svelte";
  import HeadMeta from "$lib/components/HeadMeta.svelte";
  import { DEFAULT_DESCRIPTION, SITE_NAME } from "$lib/meta";

  const { title, description = DEFAULT_DESCRIPTION, children } = $props();
  const pageTitle = $derived(title ? `${SITE_NAME} - ${title}` : SITE_NAME);
</script>

<HeadMeta title={pageTitle} {description} />

<a
  href="/"
  class="text-muted-foreground group-hover:text-muted-foreground/70 group mb-4 flex cursor-pointer items-center gap-2 transition-colors duration-200">
  <ArrowLeft class="h-4 w-4" />
  <span class="font-medium">Voltar à pesquisa</span>
</a>

<div class="prose max-w-none">
  {@render children?.()}
</div>

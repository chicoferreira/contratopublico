<script lang="ts">
  import { Pagination as PaginationPrimitive } from "bits-ui";
  import ChevronLeftIcon from "@lucide/svelte/icons/chevron-left";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { cn } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    children,
    ...restProps
  }: PaginationPrimitive.PrevButtonProps = $props();
</script>

{#snippet Fallback()}
  <ChevronLeftIcon class="size-4" />
  <span>Anterior</span>
{/snippet}

<PaginationPrimitive.PrevButton
  bind:ref
  aria-label="Ir para a página anterior"
  class={cn(
    buttonVariants({
      size: "default",
      variant: "ghost",
      class:
        "hover:bg-muted-foreground/20 dark:hover:bg-muted-foreground/30 gap-1 px-2.5 sm:pl-2.5",
    }),
    className,
  )}
  children={children || Fallback}
  {...restProps} />

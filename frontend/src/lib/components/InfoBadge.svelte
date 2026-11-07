<script lang="ts">
  import type { Snippet } from "svelte";
  import { Badge, type BadgeVariant } from "$lib/components/ui/badge";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { cn } from "$lib/utils";
  import InfoPopover from "./InfoPopover.svelte";

  let {
    title,
    variant,
    class: className,
    badge,
    children,
  }: {
    title: string;
    variant?: BadgeVariant;
    class?: string;
    badge?: Snippet;
    children: Snippet;
  } = $props();
</script>

<Popover>
  <PopoverTrigger class="inline-flex max-w-full align-middle">
    <Badge {variant} class={cn("h-6 max-w-full", className)}>
      {#if badge}
        {@render badge()}
      {:else}
        {title}
      {/if}
    </Badge>
  </PopoverTrigger>
  <PopoverContent>
    <InfoPopover {title}>
      {@render children()}
    </InfoPopover>
  </PopoverContent>
</Popover>

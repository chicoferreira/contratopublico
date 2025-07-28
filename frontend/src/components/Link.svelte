<script lang="ts">
  import { cn } from "$lib/utils";
  import { ExternalLink } from "@lucide/svelte";
  import type { Snippet } from "svelte";

  interface Props {
    url: string;
    title?: string;
    class?: string;
    showIcon?: boolean;
    external?: boolean;
    children?: Snippet;
  }

  let {
    url,
    title,
    class: extraClasses = "",
    showIcon = true,
    children,
  }: Props = $props();
</script>

<a
  href={url}
  target={"_blank"}
  rel={"noopener noreferrer"}
  class="group inline-flex w-fit shrink-0 items-center gap-[6px] transition-opacity hover:opacity-80"
  {title}>
  <p
    class={cn(
      "text-muted-foreground text-sm transition-colors group-hover:text-blue-500 group-hover:underline",
      extraClasses,
    )}>
    {#if children}
      {@render children()}
    {/if}
  </p>
  {#if showIcon}
    <ExternalLink
      size={15}
      class={cn(
        "text-muted-foreground transition-colors group-hover:text-blue-500",
        extraClasses,
      )} />
  {/if}
</a>

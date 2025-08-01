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
  <span
    class={cn(
      "text-sm text-blue-500 transition-colors group-hover:text-blue-500 group-hover:underline dark:text-blue-400",
      extraClasses,
    )}>
    {#if children}
      {@render children()}
    {/if}
  </span>
  {#if showIcon}
    <ExternalLink
      size={15}
      class={cn(
        "text-blue-500 transition-colors group-hover:text-blue-500 dark:text-blue-400",
        extraClasses,
      )} />
  {/if}
</a>

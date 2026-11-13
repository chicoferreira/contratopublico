<script lang="ts">
  import { cpvPath, loadCpvTree, type CpvTree } from "$lib/cpv";
  import { Tag } from "@lucide/svelte";
  import { onMount } from "svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import { cn } from "$lib/utils";
  import type { MatchingRange } from "$lib/types/api";
  import InfoPopoverItem from "./InfoPopoverItem.svelte";
  import Highlighted from "./Highlighted.svelte";

  let {
    cpv,
    index,
    ranges,
  }: {
    cpv: { code: string; designation: string };
    index: number;
    ranges: Record<string, MatchingRange[]>;
  } = $props();

  let tree = $state.raw<CpvTree>();
  let picked = $state<string>();

  onMount(async () => {
    tree = await loadCpvTree();
  });

  const path = $derived(tree ? cpvPath(tree, cpv.code) : [cpv.code]);
  const selected = $derived(picked ?? cpv.code);
  const isOwn = $derived(selected === cpv.code);
</script>

<InfoPopoverItem icon={Tag}>
  {#snippet title()}
    <Breadcrumb.Root>
      <Breadcrumb.List class="gap-x-1 gap-y-0 font-normal">
        {#each path as code, i (code)}
          {#if i > 0}
            <Breadcrumb.Separator />
          {/if}
          <Breadcrumb.Item>
            <button
              type="button"
              aria-current={code === selected ? "true" : undefined}
              class={cn(
                "hover:text-foreground cursor-pointer tabular-nums",
                code === selected && "text-foreground",
                code === cpv.code && "font-semibold",
              )}
              onclick={() => (picked = code)}>
              {#if code === cpv.code}
                <Highlighted content={code} ranges={ranges["cpvs.code"]} {index} />
              {:else}
                {code}
              {/if}
            </button>
          </Breadcrumb.Item>
        {/each}
      </Breadcrumb.List>
    </Breadcrumb.Root>
  {/snippet}
  {#if isOwn}
    <Highlighted content={cpv.designation} ranges={ranges["cpvs.designation"]} {index} />
  {:else}
    {tree?.names[selected]}
  {/if}
</InfoPopoverItem>

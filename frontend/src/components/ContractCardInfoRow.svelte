<script lang="ts">
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Info } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import InfoPopover from "./InfoPopover.svelte";

  let {
    Icon,
    label,
    value,
    popoverContent,
  }: {
    Icon: any;
    label: string;
    value: Snippet<[]>;
    popoverContent: Snippet<[]>;
  } = $props();
</script>

<div class="flex items-center gap-3">
  <Icon class="text-muted-foreground/80 h-6 w-6 shrink-0" />
  <div class="space-y-0">
    <Popover.Root>
      <Popover.Trigger>
        <div class="group flex items-center gap-1 transition-colors">
          <span
            class="text-muted-foreground/80 group-hover:text-muted-foreground text-sm font-semibold uppercase transition-colors">
            {label}
          </span>
          <Info
            class="text-muted-foreground/80 group-hover:text-muted-foreground h-4 w-4 transition-colors" />
        </div>
      </Popover.Trigger>
      <Popover.Content>
        <InfoPopover title={label}>
          {#snippet content()}
            <div>
              {@render popoverContent()}
            </div>
          {/snippet}
        </InfoPopover>
      </Popover.Content>
    </Popover.Root>
    <div class="font-medium break-words">
      {@render value()}
    </div>
  </div>
</div>

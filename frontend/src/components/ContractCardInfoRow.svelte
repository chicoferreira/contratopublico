<script lang="ts">
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import { Info } from "@lucide/svelte";

  let {
    Icon,
    label,
    value,
    tooltipContent,
  }: {
    Icon: any;
    label: string;
    value: string;
    tooltipContent: () => any;
  } = $props();
</script>

<div class="flex items-center gap-3">
  <Icon class="text-muted-foreground/70 h-6 w-6 shrink-0" />
  <div class="space-y-0">
    <Tooltip.Provider delayDuration={200} skipDelayDuration={0}>
      <Tooltip.Root delayDuration={200}>
        <Tooltip.Trigger class="cursor-help">
          <div
            class="group hover:bg-muted/50 flex items-center gap-1 rounded transition-colors">
            <span
              class="text-muted-foreground/70 group-hover:text-muted-foreground text-sm font-semibold uppercase transition-colors">
              {label}
            </span>
            <Info
              class="text-muted-foreground/70 group-hover:text-muted-foreground h-4 w-4 transition-colors" />
          </div>
        </Tooltip.Trigger>
        <Tooltip.Content>
          <div class="max-w-80 space-y-1">
            {@render tooltipContent()}
          </div>
        </Tooltip.Content>
      </Tooltip.Root>
    </Tooltip.Provider>
    <div class="font-medium break-words">
      {value}
    </div>
  </div>
</div>

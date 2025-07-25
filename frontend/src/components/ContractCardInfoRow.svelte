<script lang="ts">
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Info } from "@lucide/svelte";

  let {
    Icon,
    label,
    value,
    popoverContent,
  }: {
    Icon: any;
    label: string;
    value: string;
    popoverContent: () => any;
  } = $props();
</script>

<div class="flex items-center gap-3">
  <Icon class="text-muted-foreground/70 h-6 w-6 shrink-0" />
  <div class="space-y-0">
    <Popover.Root>
      <Popover.Trigger class="cursor-pointer">
        <div class="group flex items-center gap-1 transition-colors">
          <span
            class="text-muted-foreground/70 group-hover:text-muted-foreground text-sm font-semibold uppercase transition-colors">
            {label}
          </span>
          <Info
            class="text-muted-foreground/70 group-hover:text-muted-foreground h-4 w-4 transition-colors" />
        </div>
      </Popover.Trigger>
      <Popover.Content class="w-fit min-w-80 px-4 py-3">
        <div class="space-y-1 text-sm">
          {@render popoverContent()}
        </div>
      </Popover.Content>
    </Popover.Root>
    <div class="font-medium break-words">
      {value}
    </div>
  </div>
</div>

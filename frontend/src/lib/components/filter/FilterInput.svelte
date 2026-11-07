<script lang="ts">
  import Input from "$lib/components/ui/input/input.svelte";
  import FilterField from "./FilterField.svelte";

  let {
    value = $bindable(),
    labelContent,
    IconComponent,
    descriptionContent,
    onClear,
    ...extraInputClasses
  } = $props();
</script>

<FilterField {labelContent} {descriptionContent} active={value} {onClear}>
  {#snippet children(id)}
    <div class="relative">
      <IconComponent
        class="text-muted-foreground pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2" />
      <Input bind:value {...extraInputClasses} {id} class="h-9 pl-10" />
    </div>
  {/snippet}
</FilterField>

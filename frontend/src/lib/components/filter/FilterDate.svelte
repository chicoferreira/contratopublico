<script lang="ts">
  import { Calendar } from "@lucide/svelte";
  import FilterInput from "./FilterInput.svelte";

  let {
    value = $bindable(),
    labelContent,
    descriptionContent,
  }: { value: string | undefined; labelContent: string; descriptionContent: string } = $props();
</script>

<FilterInput
  bind:value
  {labelContent}
  {descriptionContent}
  IconComponent={Calendar}
  onClear={() => (value = undefined)}
  type="date"
  placeholder="Selecione uma data" />

<script lang="ts" generics="T">
  import GridCardTitle from "./GridCardTitle.svelte";
  import GridSubCardList from "./GridSubCardList.svelte";
  const {
    title,
    icon,
    elements,
    fallback,
    renderElement,
  }: {
    title?: string;
    icon?: any;
    elements: T[];
    fallback: string;
    renderElement: (elem: T) => any;
  } = $props();
</script>

<GridCardTitle title={elements.length <= 1 ? title : `${title} (${elements.length})`} {icon}>
  <GridSubCardList {elements} {fallback} {renderElement} />
</GridCardTitle>

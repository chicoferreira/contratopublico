<script lang="ts">
  import { loadLocationTree } from "$lib/locations";
  import { MapPin } from "@lucide/svelte";
  import { onMount, type Snippet } from "svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import type { Tree } from "$lib/tree";
  import type { DetailedExecutionPlace } from "$lib/types/api";
  import InfoPopoverItem from "./InfoPopoverItem.svelte";

  let {
    place,
    total,
    isDistrict,
    label,
  }: {
    place: DetailedExecutionPlace;
    total: number;
    isDistrict: boolean;
    label: Snippet<[keyof DetailedExecutionPlace]>;
  } = $props();

  let tree = $state.raw<Tree>();

  onMount(async () => {
    tree = await loadLocationTree();
  });

  const municipalityCount = $derived(
    place.region ? tree?.nodes.get(place.region.id)?.children.length : undefined,
  );
</script>

<p>
  Onde são realizadas as principais entregas, serviços ou obras do contrato, independentemente da
  sede das entidades.
</p>
<p class="text-muted-foreground/60 text-xs">
  A área indicada pode ser mais abrangente do que o local exato.
</p>
<InfoPopoverItem icon={MapPin}>
  {#snippet title()}
    <Breadcrumb.List
      class="*:last:text-foreground gap-x-1 gap-y-0 font-normal *:last:font-semibold">
      <Breadcrumb.Item>{@render label("country")}</Breadcrumb.Item>
      {#if place.region}
        <Breadcrumb.Separator />
        <Breadcrumb.Item>{@render label("region")}</Breadcrumb.Item>
      {/if}
      {#if place.municipality}
        <Breadcrumb.Separator />
        <Breadcrumb.Item>{@render label("municipality")}</Breadcrumb.Item>
      {/if}
    </Breadcrumb.List>
  {/snippet}
  <p>
    {#if place.municipality}
      {#if place.municipality.label === place.region?.label}
        Apenas o concelho, e não o distrito.
      {:else if isDistrict}
        Um dos {municipalityCount} concelhos do distrito.
      {:else}
        Um dos {municipalityCount} concelhos da região autónoma.
      {/if}
    {:else if isDistrict}
      O distrito, que inclui {municipalityCount} concelhos.
    {:else if place.region?.id === "madeira"}
      A região autónoma, que inclui {municipalityCount} concelhos nas ilhas da Madeira e do Porto Santo.
    {:else if place.region?.id === "azores"}
      A região autónoma, que inclui {municipalityCount} concelhos em nove ilhas.
    {:else if place.region?.id === "mainland"}
      O continente, mas não os Açores nem a Madeira.
    {:else if place.region?.id === "consulates"}
      Os consulados de Portugal, que ficam noutros países.
    {:else if place.country.id === "portugal"}
      O país, que inclui 18 distritos no continente e 2 regiões autónomas.
    {:else if place.country.id === "europeanUnion"}
      A União Europeia, que inclui 27 países, entre eles Portugal.
    {:else}
      Fora de Portugal, mesmo que as entidades sejam portuguesas.
    {/if}
  </p>
</InfoPopoverItem>
{#if total > 1}
  <p>É um dos {total} locais de execução deste contrato.</p>
{/if}
<!-- TODO: link to the contracts in this place in a new tab, building the URL with buildSearchParams once the location filter exists -->

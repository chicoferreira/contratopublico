<script lang="ts">
  import type { DetailedExecutionPlace, MatchingRange } from "$lib/types/api";
  import { isDistrictId } from "$lib/locations";
  import { MapPin } from "@lucide/svelte";
  import Highlighted from "./Highlighted.svelte";
  import InfoBadge from "./InfoBadge.svelte";
  import ContractPlaceDetails from "./ContractPlaceDetails.svelte";

  let {
    place,
    index,
    total,
    ranges,
  }: {
    place: DetailedExecutionPlace;
    index: number;
    total: number;
    ranges: Record<string, MatchingRange[]>;
  } = $props();

  const isDistrict = $derived(place.region !== null && isDistrictId(place.region.id));
</script>

{#snippet label(level: keyof DetailedExecutionPlace)}
  <Highlighted
    content={place[level]?.label}
    ranges={ranges[`executionPlaces.${level}.label`]}
    {index} />
{/snippet}

<InfoBadge title="Local de Execução" variant="secondary">
  {#snippet badge()}
    <MapPin aria-hidden="true" />
    <span class="max-w-56 truncate">
      {#if place.municipality}
        {@render label("municipality")}
        {#if place.municipality.label === place.region?.label}
          <span class="text-muted-foreground font-normal">(concelho)</span>
        {:else}
          <span class="text-muted-foreground font-normal">· {@render label("region")}</span>
        {/if}
      {:else if place.region}
        {@render label("region")}
        {#if isDistrict}
          <span class="text-muted-foreground font-normal">(distrito)</span>
        {/if}
      {:else}
        {@render label("country")}
      {/if}
    </span>
  {/snippet}
  <ContractPlaceDetails {place} {total} {isDistrict} {label} />
</InfoBadge>

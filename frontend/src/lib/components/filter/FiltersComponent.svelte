<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import {
    Hash,
    Euro,
    Building,
    X as XIcon,
    Funnel,
    Lightbulb,
    CalendarDays,
    Shapes,
  } from "@lucide/svelte";
  import type { Filters } from "$lib/types/api";
  import { MAX_ENTITY_FILTER_LENGTH } from "$lib";
  import FilterContractTypes from "./FilterContractTypes.svelte";
  import FilterCpvs from "./FilterCpvs.svelte";
  import FilterDate from "./FilterDate.svelte";
  import FilterInput from "./FilterInput.svelte";
  import FilterProcedureTypes from "./FilterProcedureTypes.svelte";
  import FilterSection from "./FilterSection.svelte";
  import { fade } from "svelte/transition";

  let {
    filters = $bindable(),
    activeFiltersCount,
    filtersOpen = $bindable(),
  }: { filters: Filters; activeFiltersCount: number; filtersOpen: boolean } = $props();

  let displayMinPrice = $state("");
  let displayMaxPrice = $state("");

  function closeFiltersPanel() {
    filtersOpen = false;
  }

  const priceFormatter = new Intl.NumberFormat("pt-PT", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });

  $effect(() => {
    if (filters.minPrice || filters.minPrice === 0) {
      displayMinPrice = priceFormatter.format(filters.minPrice / 100);
    } else {
      displayMinPrice = "";
    }

    if (filters.maxPrice || filters.maxPrice === 0) {
      displayMaxPrice = priceFormatter.format(filters.maxPrice / 100);
    } else {
      displayMaxPrice = "";
    }
  });

  $effect(() => {
    const MAX_CONTRACT_ID = 100000000;
    const MAX_PRICE_CENTS = 1000000000000000;

    if (filters.minId && filters.minId > MAX_CONTRACT_ID) {
      filters.minId = MAX_CONTRACT_ID;
    }

    if (filters.maxId && filters.maxId > MAX_CONTRACT_ID) {
      filters.maxId = MAX_CONTRACT_ID;
    }

    if (filters.minPrice && filters.minPrice > MAX_PRICE_CENTS) {
      filters.minPrice = MAX_PRICE_CENTS;
    }

    if (filters.maxPrice && filters.maxPrice > MAX_PRICE_CENTS) {
      filters.maxPrice = MAX_PRICE_CENTS;
    }
  });

  function handlePriceBlur(e: Event, target: "minPrice" | "maxPrice") {
    const input = e.currentTarget as HTMLInputElement;
    const raw = input.value.replace(/\s/g, "").replace(",", ".");
    const num = parseFloat(raw);
    if (!isNaN(num)) {
      const cents = Math.round(num * 100);
      (filters as any)[target] = cents;
      input.value = priceFormatter.format(cents / 100);
    } else {
      delete (filters as any)[target];
      input.value = "";
    }
  }

  function handlePriceFocus(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    input.value = input.value.replace(/\s/g, "").replace(".", "");
  }

  function clearAllFilters() {
    filters = {};
  }

  $effect(() => {
    Object.keys(filters).forEach((key) => {
      let value = filters[key as keyof typeof filters];
      if (!value && value !== 0) {
        delete filters[key as keyof typeof filters];
      }
    });
  });
</script>

<div class="bg-card rounded-md border px-6 py-5">
  <div class="flex items-center pb-4">
    <div class="flex flex-grow items-center gap-3 text-lg font-semibold">
      <button
        type="button"
        class="group flex cursor-pointer items-center gap-3 text-left"
        onclick={closeFiltersPanel}
        aria-label="Fechar filtros">
        <Funnel class="h-5 w-5" />
        <span class="group-hover:underline">Filtros Avançados</span>
      </button>
      {#if activeFiltersCount > 0}
        <span
          class="bg-primary text-primary-foreground hidden rounded-full px-2 py-0.5 text-xs font-medium md:block"
          transition:fade={{ duration: 100 }}>
          {activeFiltersCount}
          {activeFiltersCount === 1 ? "ativo" : "ativos"}
        </span>
      {/if}
    </div>
    {#if activeFiltersCount > 0}
      <div class="flex justify-end" transition:fade={{ duration: 100 }}>
        <Button
          variant="ghost"
          size="sm"
          onclick={clearAllFilters}
          class="text-muted-foreground hover:text-foreground !h-7">
          <XIcon class="mr-1 h-4 w-4" />
          Limpar Tudo
        </Button>
      </div>
    {/if}
  </div>

  <div class="border-muted bg-muted mb-4 flex items-center gap-2 rounded-lg border p-3">
    <Lightbulb class="h-5 w-5 flex-shrink-0 text-yellow-500 dark:text-yellow-400" />
    <div>
      <span class="font-medium">Dica:</span>
      <span class="text-muted-foreground ml-1">
        Os filtros podem não ser necessários na maioria dos casos. Para resultados mais precisos,
        escreva termos "entre aspas" na barra de pesquisa para obter correspondências exatas.
      </span>
    </div>
  </div>

  <div class="space-y-4">
    <FilterSection
      title="Número de Identificador"
      IconComponent={Hash}
      filter={filters}
      fields={["maxId", "minId"]}>
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        <FilterInput
          labelContent="Número de Identificador Mínimo"
          IconComponent={Hash}
          descriptionContent="O número de identificador mínimo do contrato"
          bind:value={filters.minId}
          onClear={() => delete filters.minId}
          placeholder="Ex: 1000000"
          type="number"
          min="0"
          max="10000000000" />
        <FilterInput
          labelContent="Número de Identificador Máximo"
          IconComponent={Hash}
          descriptionContent="O número de identificador máximo do contrato"
          bind:value={filters.maxId}
          onClear={() => delete filters.maxId}
          placeholder="Ex: 2000000"
          type="number"
          min="0"
          max="10000000000" />
      </div>
    </FilterSection>

    <Separator />

    <FilterSection
      title="Datas"
      IconComponent={CalendarDays}
      filter={filters}
      fields={["startPublicationDate", "endPublicationDate", "startSigningDate", "endSigningDate"]}>
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        <FilterDate
          labelContent="Publicado desde"
          bind:value={filters.startPublicationDate}
          descriptionContent="Data inicial de publicação no Portal BASE" />
        <FilterDate
          labelContent="Publicado até"
          bind:value={filters.endPublicationDate}
          descriptionContent="Data final de publicação no Portal BASE" />
        <FilterDate
          labelContent="Assinado desde"
          bind:value={filters.startSigningDate}
          descriptionContent="Data inicial de assinatura do contrato" />
        <FilterDate
          labelContent="Assinado até"
          bind:value={filters.endSigningDate}
          descriptionContent="Data final de assinatura do contrato" />
      </div>
    </FilterSection>

    <Separator />

    <FilterSection
      title="Valor Contratual Inicial"
      IconComponent={Euro}
      filter={filters}
      fields={["minPrice", "maxPrice"]}>
      <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
        <FilterInput
          bind:value={displayMinPrice}
          onClear={() => delete filters.minPrice}
          labelContent="Valor Contratual Mínimo"
          IconComponent={Euro}
          type="text"
          placeholder="10,00"
          descriptionContent="Valor inicial mínimo do contrato"
          onfocus={handlePriceFocus}
          onblur={(e: Event) => handlePriceBlur(e, "minPrice")} />
        <FilterInput
          bind:value={displayMaxPrice}
          onClear={() => delete filters.maxPrice}
          labelContent="Valor Contratual Máximo"
          IconComponent={Euro}
          type="text"
          placeholder="100,00"
          descriptionContent="Valor inicial máximo do contrato"
          onfocus={handlePriceFocus}
          onblur={(e: Event) => handlePriceBlur(e, "maxPrice")} />
      </div>
    </FilterSection>

    <Separator />

    <FilterSection
      title="Entidades Relacionadas"
      IconComponent={Building}
      filter={filters}
      fields={["contracting", "contracted"]}>
      <div class="space-y-4">
        <FilterInput
          bind:value={filters.contracting}
          onClear={() => delete filters.contracting}
          labelContent="Entidade Contratante (Exata)"
          IconComponent={Building}
          descriptionContent="NIF (recomendado) ou nome completo exato da entidade pública responsável pela contratação (adjudicante)"
          type="text"
          maxlength={MAX_ENTITY_FILTER_LENGTH}
          placeholder="Ex: Câmara Municipal de Lisboa ou 500051070" />
        <FilterInput
          bind:value={filters.contracted}
          onClear={() => delete filters.contracted}
          labelContent="Entidade Contratada (Exata)"
          IconComponent={Building}
          descriptionContent="NIF (recomendado) ou nome completo exato da entidade selecionada para a prestação de serviços (adjudicatária)"
          type="text"
          maxlength={MAX_ENTITY_FILTER_LENGTH}
          placeholder="Ex: Empresa XPTO, Lda. ou 123456789" />
      </div>
    </FilterSection>

    <Separator />

    <FilterSection
      title="Classificação"
      IconComponent={Shapes}
      filter={filters}
      fields={["contractTypes", "procedureTypes", "cpvs"]}
      addedDate="2026-11-13">
      <div class="space-y-4">
        <FilterContractTypes bind:value={filters.contractTypes} />
        <FilterProcedureTypes bind:value={filters.procedureTypes} />
        <FilterCpvs bind:value={filters.cpvs} />
      </div>
    </FilterSection>
  </div>
</div>

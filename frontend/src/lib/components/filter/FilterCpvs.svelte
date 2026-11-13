<script lang="ts">
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import Highlighted from "$lib/components/Highlighted.svelte";
  import { MAX_CPV_FILTERS } from "$lib";
  import {
    codeMatchRanges,
    cpvState,
    cpvView,
    loadCpvTree,
    nameMatchRanges,
    searchCpvs,
    toggleCpv,
    type CpvTree,
  } from "$lib/cpv";
  import { cn } from "$lib/utils.js";
  import { Check, ChevronDown, ChevronRight, Minus, Tag } from "@lucide/svelte";
  import { onMount } from "svelte";
  import { SvelteSet } from "svelte/reactivity";
  import { blur, slide } from "svelte/transition";
  import FilterChip from "./FilterChip.svelte";
  import FilterField from "./FilterField.svelte";

  let { value = $bindable() }: { value: string[] | undefined } = $props();

  const RESULT_LIMIT = 60;
  const treeId = $props.id();

  let tree = $state.raw<CpvTree>();
  let loadError = $state(false);
  let query = $state("");
  let open = $state(false);
  let active = $state(0);
  let animate = $state(false);
  let field = $state<HTMLElement>();
  let input = $state<HTMLInputElement>();
  const expanded = new SvelteSet<string>();
  const searchToggled = new SvelteSet<string>();

  onMount(async () => {
    tree = await loadCpvTree();
    loadError = !tree;
  });

  const selected = $derived(value ?? []);
  const hits = $derived(tree && query.trim() ? searchCpvs(tree, query) : null);
  const toggled = $derived(hits ? searchToggled : expanded);
  const view = $derived(tree && cpvView(tree, hits?.slice(0, RESULT_LIMIT) ?? null, toggled));
  const rows = $derived(view?.rows ?? []);
  const rowIndex = $derived(new Map(rows.map((code, i) => [code, i])));
  const index = $derived(Math.max(0, Math.min(active, rows.length - 1)));
  const activeRow = $derived(rows[index]);
  const statusText = $derived.by(() => {
    if (loadError) return "Não foi possível carregar os CPVs.";
    if (!tree) return "A carregar CPVs...";
    if (!hits) return `${tree.parent.size.toLocaleString("pt-PT")} códigos`;
    const total = `${hits.length.toLocaleString("pt-PT")} ${hits.length === 1 ? "resultado" : "resultados"}`;
    return hits.length > RESULT_LIMIT
      ? `A mostrar ${RESULT_LIMIT} de ${total}, escreva mais para filtrar`
      : total;
  });

  function setQuery(text: string) {
    animate = false;
    query = text;
    active = 0;
    searchToggled.clear();
    open = true;
  }

  function toggle(code: string) {
    const next = toggleCpv(code, selected, MAX_CPV_FILTERS);
    value = next.length ? next : undefined;
  }

  function toggleOpen(code: string) {
    animate = true;
    if (!toggled.delete(code)) toggled.add(code);
  }

  function moveTo(i: number) {
    active = i;
    document.getElementById(`${treeId}-${rows[i]}`)?.scrollIntoView({ block: "nearest" });
  }

  function moveBy(step: number) {
    if (!open) open = true;
    else if (rows.length) moveTo((index + step + rows.length) % rows.length);
    return true;
  }

  function selectActive() {
    if (!open) open = true;
    else if (activeRow) toggle(activeRow);
    return true;
  }

  function treeArrowTarget() {
    const caretAtEnd = input?.selectionStart === query.length;
    return open && caretAtEnd ? activeRow : undefined;
  }

  function expandOrDescend() {
    const row = treeArrowTarget();
    if (!row || !tree?.children.has(row)) return false;
    if (view?.isOpen(row)) moveTo(index + 1);
    else toggleOpen(row);
    return true;
  }

  function collapseOrAscend() {
    const row = treeArrowTarget();
    if (!row) return false;
    if (view?.isOpen(row)) toggleOpen(row);
    else moveTo(rowIndex.get(tree?.parent.get(row) ?? "") ?? index);
    return true;
  }

  function removeLastChip() {
    if (query || !selected.length) return false;
    value = selected.length > 1 ? selected.slice(0, -1) : undefined;
    return true;
  }

  function clearOrClose() {
    if (query) setQuery("");
    else if (open) open = false;
    else return false;
    return true;
  }

  const keyHandlers: Record<string, () => boolean> = {
    ArrowDown: () => moveBy(1),
    ArrowUp: () => moveBy(-1),
    Enter: selectActive,
    ArrowRight: expandOrDescend,
    ArrowLeft: collapseOrAscend,
    Backspace: removeLastChip,
    Escape: clearOrClose,
  };

  function onkeydown(e: KeyboardEvent) {
    if (keyHandlers[e.key]?.()) e.preventDefault();
  }
</script>

{#snippet branch(codes: string[], depth: number)}
  {#each codes as code (code)}
    {@const i = rowIndex.get(code)}
    {@const state = cpvState(code, selected)}
    {@const checked = state === "on" || state === "covered"}
    {@const opened = view!.isOpen(code)}
    {@const name = tree!.names[code]}
    <!-- svelte-ignore a11y_click_events_have_key_events (keys are handled by the combobox input) -->
    <div
      id="{treeId}-{code}"
      role="treeitem"
      tabindex="-1"
      aria-level={depth + 1}
      aria-expanded={tree!.children.has(code) ? opened : undefined}
      aria-selected={checked}
      class={cn(
        "flex cursor-pointer items-start gap-2 rounded-sm px-2 py-1.5",
        i === index && "bg-accent text-accent-foreground",
        hits && !view!.matched.has(code) && "text-muted-foreground",
      )}
      onclick={() => toggle(code)}
      onmouseenter={() => i !== undefined && (active = i)}>
      {#if tree!.children.has(code)}
        <button
          type="button"
          tabindex="-1"
          aria-label="{opened ? 'Recolher' : 'Expandir'} {code}"
          class="text-muted-foreground hover:bg-foreground/10 hover:text-foreground grid size-5 shrink-0 cursor-pointer place-items-center rounded-sm"
          onclick={(e) => {
            e.stopPropagation();
            toggleOpen(code);
          }}>
          <ChevronRight
            class={cn("size-4 transition-transform duration-200", opened && "rotate-90")} />
        </button>
      {:else}
        <span class="size-5 shrink-0"></span>
      {/if}
      <span
        class={cn(
          "border-input text-primary-foreground mt-0.5 grid size-4 shrink-0 place-items-center rounded-[4px] border shadow-xs",
          state !== "off" && "bg-primary border-primary",
          state === "covered" && "opacity-50",
        )}>
        {#if checked}
          <Check class="size-3.5" />
        {:else if state === "mixed"}
          <Minus class="size-3.5" />
        {/if}
      </span>
      <span class="min-w-0">
        <Highlighted
          class="font-semibold tabular-nums"
          content={code}
          ranges={codeMatchRanges(code, query)} />
        <Highlighted
          content={name}
          ranges={view!.matched.has(code) ? nameMatchRanges(name, query) : []} />
      </span>
    </div>
    {#if opened}
      <div role="group" transition:slide={{ duration: animate ? 200 : 0 }}>
        <div
          class="border-muted-foreground/35 ml-4.5 border-l pl-1"
          transition:blur={{ duration: animate ? 200 : 0 }}>
          {@render branch(view!.childrenOf(code), depth + 1)}
        </div>
      </div>
    {/if}
  {/each}
{/snippet}

<FilterField
  labelContent="CPV (Vocabulário Comum para os Contratos Públicos)"
  descriptionContent="Mostra contratos com pelo menos um dos códigos selecionados ou das suas subcategorias"
  active={!!value}
  onClear={() => (value = undefined)}>
  {#snippet children(id)}
    <Popover.Root bind:open>
      <div
        bind:this={field}
        class={cn(
          buttonVariants({ variant: "outline" }),
          "h-auto min-h-9 w-full cursor-text flex-wrap justify-start py-1.5 font-normal",
          "focus-within:border-ring focus-within:ring-ring/50 focus-within:ring-[3px]",
        )}>
        <Tag class="text-muted-foreground" />
        {#each selected as code (code)}
          {@const name = tree?.names[code]}
          <FilterChip label={code} onRemove={() => toggle(code)}>
            {code}
            {#if name}
              <span class="text-muted-foreground max-w-48 truncate font-normal">{name}</span>
            {/if}
          </FilterChip>
        {/each}
        <input
          {id}
          bind:this={input}
          role="combobox"
          aria-expanded={open}
          aria-controls={treeId}
          aria-activedescendant={open && activeRow ? `${treeId}-${activeRow}` : undefined}
          autocomplete="off"
          class="placeholder:text-muted-foreground min-w-32 flex-1 bg-transparent text-ellipsis outline-none"
          placeholder="Designação ou código (ex: obras, 45000000)..."
          value={query}
          oninput={(e) => setQuery(e.currentTarget.value)}
          onclick={() => (open = true)}
          onblur={() => (open = false)}
          {onkeydown} />
        <ChevronDown class="ml-auto opacity-50" />
      </div>
      <Popover.Content
        customAnchor={field}
        align="start"
        avoidCollisions={false}
        trapFocus={false}
        onOpenAutoFocus={(e) => e.preventDefault()}
        onCloseAutoFocus={(e) => e.preventDefault()}
        onEscapeKeydown={(e) => e.preventDefault()}
        onInteractOutside={(e) => field?.contains(e.target as Node) && e.preventDefault()}
        onmousedown={(e) => e.preventDefault()}
        class="w-(--bits-popover-anchor-width) space-y-0 p-0">
        <div class="text-muted-foreground border-b px-3 py-2 text-xs">{statusText}</div>
        <div
          id={treeId}
          role="tree"
          aria-label="CPVs"
          aria-multiselectable="true"
          class="max-h-80 overflow-y-auto p-1">
          {@render branch(view?.roots ?? [], 0)}
          {#if hits && !rows.length}
            <p class="text-muted-foreground px-3 py-6 text-center">Nenhum CPV encontrado.</p>
          {/if}
        </div>
      </Popover.Content>
    </Popover.Root>
  {/snippet}
</FilterField>

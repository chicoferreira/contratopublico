<script lang="ts">
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { selectionState, toggleSelection, treeView, type Tree, type TreeNode } from "$lib/tree";
  import { cn } from "$lib/utils.js";
  import { Check, ChevronDown, ChevronRight, Minus } from "@lucide/svelte";
  import { onMount, untrack, type Component, type Snippet } from "svelte";
  import { SvelteSet } from "svelte/reactivity";
  import { blur, slide } from "svelte/transition";
  import FilterChip from "./FilterChip.svelte";
  import FilterField from "./FilterField.svelte";

  let {
    value = $bindable(),
    max,
    load,
    search,
    labelContent,
    addedDate,
    descriptionContent,
    IconComponent,
    placeholder,
    treeLabel,
    countLabel,
    loadingText,
    errorText,
    emptyText,
    initiallyExpanded = [],
    nodeLabel = (node) => node.id,
    row,
    chip,
  }: {
    value: string[] | undefined;
    max: number;
    load: () => Promise<Tree | undefined>;
    search: (tree: Tree, query: string) => TreeNode[];
    labelContent: string;
    addedDate?: string;
    descriptionContent: string;
    IconComponent: Component;
    placeholder: string;
    treeLabel: string;
    countLabel: string;
    loadingText: string;
    errorText: string;
    emptyText: string;
    initiallyExpanded?: string[];
    nodeLabel?: (node: TreeNode) => string;
    row: Snippet<[node: TreeNode, query: string, matched: boolean]>;
    chip: Snippet<[id: string, node: TreeNode | undefined, loading: boolean]>;
  } = $props();

  const RESULT_LIMIT = 60;
  const treeId = $props.id();

  let tree = $state.raw<Tree>();
  let loadError = $state(false);
  let query = $state("");
  let open = $state(false);
  let highlighted = $state.raw<TreeNode>();
  let animate = $state(false);
  let field = $state<HTMLElement>();
  let input = $state<HTMLInputElement>();
  const expanded = new SvelteSet(untrack(() => initiallyExpanded));
  const toggledWhileSearching = new SvelteSet<string>();

  onMount(async () => {
    tree = await load();
    loadError = !tree;
  });

  const selected = $derived(value ?? []);
  const matches = $derived(tree && query.trim() ? search(tree, query) : undefined);
  const toggled = $derived(matches ? toggledWhileSearching : expanded);
  const view = $derived(tree && treeView(tree, matches?.slice(0, RESULT_LIMIT), toggled));
  const rows = $derived(view?.rows ?? []);

  const activeNode = $derived.by(() => {
    if (highlighted && rows.includes(highlighted)) return highlighted;
    const bestMatch = matches?.[0];
    if (bestMatch && rows.includes(bestMatch)) return bestMatch;
    return rows.at(0);
  });

  const statusText = $derived.by(() => {
    if (loadError) return errorText;
    if (!tree) return loadingText;
    if (!matches) return `${tree.nodes.size.toLocaleString("pt-PT")} ${countLabel}`;

    const resultWord = matches.length === 1 ? "resultado" : "resultados";
    const total = `${matches.length.toLocaleString("pt-PT")} ${resultWord}`;
    if (matches.length <= RESULT_LIMIT) return total;
    return `A mostrar ${RESULT_LIMIT} de ${total}, escreva mais para filtrar`;
  });

  function setSelected(ids: string[]) {
    value = ids.length > 0 ? ids : undefined;
  }

  function toggle(id: string) {
    setSelected(toggleSelection(tree, id, selected, max));
  }

  function toggleExpanded(node: TreeNode) {
    animate = true;
    highlighted = node;
    if (toggled.has(node.id)) toggled.delete(node.id);
    else toggled.add(node.id);
  }

  function setQuery(text: string) {
    animate = false;
    query = text;
    highlighted = undefined;
    toggledWhileSearching.clear();
    open = true;
    requestAnimationFrame(() => {
      if (activeNode) centerInTree(activeNode);
    });
  }

  // Not scrollIntoView, which would also scroll the page while typing
  function centerInTree(node: TreeNode) {
    const list = document.getElementById(treeId);
    const row = document.getElementById(`${treeId}-${node.id}`);
    if (!list || !row) return;
    const offset = row.getBoundingClientRect().top - list.getBoundingClientRect().top;
    list.scrollTop += offset - list.clientHeight / 2;
  }

  function moveTo(node: TreeNode) {
    highlighted = node;
    document.getElementById(`${treeId}-${node.id}`)?.scrollIntoView({ block: "nearest" });
  }

  function moveBy(step: number) {
    if (!open) {
      open = true;
    } else if (activeNode) {
      const next = rows.indexOf(activeNode) + step;
      moveTo(rows[(next + rows.length) % rows.length]);
    }
    return true;
  }

  function selectActive() {
    if (!open) open = true;
    else if (activeNode) toggle(activeNode.id);
    return true;
  }

  function treeArrowTarget() {
    const caretAtEnd = input?.selectionStart === query.length;
    return open && caretAtEnd ? activeNode : undefined;
  }

  function expandOrDescend() {
    const node = treeArrowTarget();
    if (!node || node.children.length === 0) return false;
    if (view?.isOpen(node)) moveBy(1);
    else toggleExpanded(node);
    return true;
  }

  function collapseOrAscend() {
    const node = treeArrowTarget();
    if (!node) return false;
    if (view?.isOpen(node)) toggleExpanded(node);
    else moveTo(node.parent ?? node);
    return true;
  }

  function removeLastChip() {
    if (query || selected.length === 0) return false;
    setSelected(selected.slice(0, -1));
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

  function highlight(node: TreeNode) {
    // Rows that are sliding out still get mouse events
    if (rows.includes(node)) highlighted = node;
  }

  function onkeydown(e: KeyboardEvent) {
    if (keyHandlers[e.key]?.()) e.preventDefault();
  }

  function onInteractOutside(e: PointerEvent) {
    if (field?.contains(e.target as Node)) e.preventDefault();
  }
</script>

{#snippet branch(view: ReturnType<typeof treeView>, nodes: TreeNode[], level: number)}
  {#each nodes as node (node.id)}
    {@const state = selectionState(tree, node.id, selected)}
    {@const checked = state === "selected" || state === "covered"}
    {@const opened = view.isOpen(node)}
    {@const expandable = node.children.length > 0}
    {@const matched = view.matched.has(node)}
    <!-- svelte-ignore a11y_click_events_have_key_events (keys are handled by the combobox input) -->
    <div
      id="{treeId}-{node.id}"
      role="treeitem"
      tabindex="-1"
      aria-level={level}
      aria-expanded={expandable ? opened : undefined}
      aria-selected={checked}
      class={cn(
        "flex cursor-pointer items-start gap-2 rounded-sm px-2 py-1.5",
        node === activeNode && "bg-accent text-accent-foreground",
        matches && !matched && "text-muted-foreground",
      )}
      onclick={() => toggle(node.id)}
      onmouseenter={() => highlight(node)}>
      {#if expandable}
        <button
          type="button"
          tabindex="-1"
          aria-label="{opened ? 'Recolher' : 'Expandir'} {nodeLabel(node)}"
          class="text-muted-foreground hover:bg-foreground/10 hover:text-foreground grid size-5 shrink-0 cursor-pointer place-items-center rounded-sm"
          onclick={(e) => {
            e.stopPropagation();
            toggleExpanded(node);
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
          state !== "unselected" && "bg-primary border-primary",
          state === "covered" && "opacity-50",
        )}>
        {#if checked}
          <Check class="size-3.5" />
        {:else if state === "partial"}
          <Minus class="size-3.5" />
        {/if}
      </span>
      <span class="min-w-0">
        {@render row(node, query, matched)}
      </span>
    </div>
    {#if opened}
      <div role="group" transition:slide={{ duration: animate ? 200 : 0 }}>
        <div
          class="border-muted-foreground/35 ml-4.5 border-l pl-1"
          transition:blur={{ duration: animate ? 200 : 0 }}>
          {@render branch(view, view.childrenOf(node), level + 1)}
        </div>
      </div>
    {/if}
  {/each}
{/snippet}

<FilterField
  {labelContent}
  {addedDate}
  {descriptionContent}
  active={selected.length > 0}
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
        <IconComponent class="text-muted-foreground" />
        {#each selected as selectedId (selectedId)}
          {@const node = tree?.nodes.get(selectedId)}
          <FilterChip
            label={node ? nodeLabel(node) : selectedId}
            onRemove={() => toggle(selectedId)}>
            {@render chip(selectedId, node, !tree && !loadError)}
          </FilterChip>
        {/each}
        <input
          {id}
          bind:this={input}
          role="combobox"
          aria-expanded={open}
          aria-controls={treeId}
          aria-activedescendant={open && activeNode ? `${treeId}-${activeNode.id}` : undefined}
          autocomplete="off"
          class="placeholder:text-muted-foreground min-w-32 flex-1 bg-transparent text-ellipsis outline-none"
          {placeholder}
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
        {onInteractOutside}
        onmousedown={(e) => e.preventDefault()}
        class="w-(--bits-popover-anchor-width) space-y-0 p-0">
        <div class="text-muted-foreground border-b px-3 py-2 text-xs">{statusText}</div>
        <div
          id={treeId}
          role="tree"
          aria-label={treeLabel}
          aria-multiselectable="true"
          class="max-h-80 overflow-y-auto p-1">
          {#if view}
            {@render branch(view, view.roots, 1)}
          {/if}
          {#if matches?.length === 0}
            <p class="text-muted-foreground px-3 py-6 text-center">{emptyText}</p>
          {/if}
        </div>
      </Popover.Content>
    </Popover.Root>
  {/snippet}
</FilterField>

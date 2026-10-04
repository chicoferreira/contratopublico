<script lang="ts">
  import * as Pagination from "$lib/components/ui/pagination/index.js";

  let {
    page = $bindable(),
    total = $bindable(),
    hitsPerPage = $bindable(),
    onPageChange,
  }: {
    page: number;
    total: number;
    hitsPerPage: number;
    onPageChange?: (newPage: number) => void;
  } = $props();

  function getPage() {
    return page;
  }

  function setPage(newPage: number) {
    onPageChange?.(newPage);
    page = newPage;
  }
</script>

<Pagination.Root count={total} perPage={hitsPerPage} bind:page={getPage, setPage}>
  {#snippet children({ pages, currentPage })}
    <Pagination.Content>
      <Pagination.Item>
        <Pagination.PrevButton />
      </Pagination.Item>
      {#each pages as page, i (page.key)}
        {#if page.type === "ellipsis"}
          <Pagination.Item>
            <Pagination.Ellipsis />
          </Pagination.Item>
        {:else}
          <!-- Hide pages next to the ellipsis on small screens -->
          {@const nextToEllipsis =
            i > 0 &&
            i < pages.length - 1 &&
            (pages[i - 1].type === "ellipsis" || pages[i + 1].type === "ellipsis")}
          <Pagination.Item class={nextToEllipsis ? "max-sm:hidden" : undefined}>
            <Pagination.Link
              {page}
              isActive={currentPage === page.value}
              size="default"
              class="hover:bg-muted-foreground/20 dark:hover:bg-muted-foreground/30 h-11 min-w-11 px-1.5">
              {page.value}
            </Pagination.Link>
          </Pagination.Item>
        {/if}
      {/each}
      <Pagination.Item>
        <Pagination.NextButton />
      </Pagination.Item>
    </Pagination.Content>
  {/snippet}
</Pagination.Root>

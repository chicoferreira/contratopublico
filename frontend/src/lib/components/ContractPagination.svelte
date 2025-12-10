<script lang="ts">
  import * as Pagination from "$lib/components/ui/pagination/index.js";

  let {
    page = $bindable(),
    total = $bindable(),
    hitsPerPage = $bindable(),
    scrolToElement = undefined,
    scrollOffset = 0,
  } = $props();

  function getPage() {
    return page;
  }

  function setPage(newPage: number) {
    if (newPage > page && scrolToElement) {
      const targetElement = document.querySelector(scrolToElement) as HTMLElement | null;
      if (targetElement) {
        const elementPosition = targetElement.getBoundingClientRect().top;
        const target = elementPosition + window.scrollY + scrollOffset;
        window.scrollTo({ top: target, behavior: "smooth" });
      }
    }
    page = newPage;
  }
</script>

<Pagination.Root count={total} perPage={hitsPerPage} bind:page={getPage, setPage}>
  {#snippet children({ pages, currentPage })}
    <Pagination.Content>
      <Pagination.Item>
        <Pagination.PrevButton />
      </Pagination.Item>
      {#each pages as page (page.key)}
        {#if page.type === "ellipsis"}
          <Pagination.Item>
            <Pagination.Ellipsis />
          </Pagination.Item>
        {:else}
          <Pagination.Item>
            <Pagination.Link
              {page}
              isActive={currentPage === page.value}
              size="icon"
              class="hover:bg-muted-foreground/20 dark:hover:bg-muted-foreground/30 size-11">
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

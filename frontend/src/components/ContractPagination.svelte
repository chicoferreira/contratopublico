<script lang="ts">
  import * as Pagination from "$lib/components/ui/pagination/index.js";

  let {
    page = $bindable(),
    total = $bindable(),
    hitsPerPage = $bindable(),
  } = $props();
</script>

<Pagination.Root count={total} perPage={hitsPerPage} bind:page>
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
              size="default"
              class="hover:bg-muted-foreground/20 dark:hover:bg-muted-foreground/30">
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

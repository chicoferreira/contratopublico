import { searchContracts, DEFAULT_SEARCH_REQUEST } from "$lib/index";
import type { SearchContractsRequest, SearchContractsResponse } from "$lib/types/api";
import { Sort } from "$lib/types/api";
import { validateEnumOrDefault } from "$lib/utils";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, url }) => {
  const params = url.searchParams;

  const query = params.get("query") || DEFAULT_SEARCH_REQUEST.query;
  const sortField = validateEnumOrDefault(
    params.get("sortField"),
    Sort.fields,
    DEFAULT_SEARCH_REQUEST.sort!.field,
  );
  const sortDirection = validateEnumOrDefault(
    params.get("sortDirection"),
    Sort.directions,
    DEFAULT_SEARCH_REQUEST.sort!.direction,
  );
  const pageParam = params.get("page");
  let page = pageParam ? parseInt(pageParam, 10) : DEFAULT_SEARCH_REQUEST.page!;
  page = Math.max(1, page);

  const sort: Sort.SortBy = { field: sortField, direction: sortDirection };
  const request: Required<SearchContractsRequest> = { query, sort, page };

  try {
    const response = await searchContracts(request, fetch);
    return {
      response,
      request,
      error: null,
    };
  } catch (error) {
    console.error(error);
    
    return {
      response: {
        contracts: [],
        total: 0,
        page: 0,
        totalPages: 0,
        elapsedMillis: 0,
        hitsPerPage: 0,
      } as SearchContractsResponse,
      request,
      error: error instanceof Error ? error.message : "Erro desconhecido",
    };
  }
};

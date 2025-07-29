import { searchContracts, DEFAULT_SEARCH_REQUEST } from "$lib/index";
import type { SearchContractsRequest } from "$lib/types/api";
import { Sort } from "$lib/types/api";
import { validateEnumOrDefault } from "$lib/utils";
import { browser } from "$app/environment";
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

  const sort: Sort.SortBy = {
    field: sortField,
    direction: sortDirection,
  };

  const request: SearchContractsRequest = {
    query,
    sort,
    page,
  };

  return {
    contracts: await searchContracts(request, fetch),
    sort,
    page,
    query,
  };
};

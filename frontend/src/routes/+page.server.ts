import { searchContracts, DEFAULT_SEARCH_REQUEST } from "$lib/index";
import type { SearchContractsRequest } from "$lib/types/api";
import { Sort } from "$lib/types/api";
import type { PageServerLoad } from "./$types";
import { env } from "$env/dynamic/private";
import { validateEnumOrDefault } from "$lib/utils";

export const load: PageServerLoad = async ({ url }) => {
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

  const backendURL = env.BACKEND_URL || "http://localhost:3000";

  return {
    contracts: await searchContracts(request, backendURL),
    sort,
    page,
    query,
  };
};

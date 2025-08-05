import { searchContracts, DEFAULT_SEARCH_REQUEST, parseSearchRequestFromParams } from "$lib/index";
import type { SearchContractsResponse } from "$lib/types/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, url }) => {
  const params = url.searchParams;
  const request = parseSearchRequestFromParams(params);

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

import { fetchStatistics } from "$lib/index";
import { searchContracts, parseSearchRequestFromParams } from "$lib/index";
import type { SearchContractsResponse, Statistics } from "$lib/types/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, url }) => {
  const params = url.searchParams;
  const searchRequest = parseSearchRequestFromParams(params);

  try {
    const [searchResponse, statisticsResponse] = await Promise.all([
      searchContracts(searchRequest, fetch),
      fetchStatistics(fetch),
    ]);

    return {
      searchResponse,
      searchRequest,
      statisticsResponse,
      error: null,
    };
  } catch (error) {
    console.error(error);

    return {
      searchResponse: {
        contracts: [],
        total: 0,
        page: 0,
        totalPages: 0,
        elapsedMillis: 0,
        hitsPerPage: 0,
      } as SearchContractsResponse,
      statisticsResponse: {
        totalSpentLast365Days: 0,
        contractsLast365Days: 0,
        totalSpentLast30Days: 0,
        contractsLast30Days: 0,
        totalSpentLast7Days: 0,
        contractsLast7Days: 0,
      } as Statistics,
      searchRequest,
      error: error instanceof Error ? error.message : "Erro desconhecido",
    };
  }
};

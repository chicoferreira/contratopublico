import { fetchStatistics, parseSearchRequestFromParams, searchContracts } from "$lib/index";
import type { SearchContractsResponse, Statistics } from "$lib/types/api";
import type { PageLoad } from "./$types";

const EMPTY_SEARCH_RESPONSE: SearchContractsResponse = {
  contracts: [],
  total: 0,
  page: 1,
  totalPages: 0,
  elapsedMillis: 0,
  hitsPerPage: 0,
};

const EMPTY_STATISTICS: Statistics = {
  totalSpentLast365Days: 0,
  contractsLast365Days: 0,
  totalSpentLast30Days: 0,
  contractsLast30Days: 0,
  totalSpentLast7Days: 0,
  contractsLast7Days: 0,
};

const STATISTICS_TTL_MS = 5 * 60 * 1000;

let cachedStatistics: Statistics | null = null;
let cachedAt = 0;

async function getCachedStatistics(fetchFn = fetch): Promise<Statistics> {
  const now = Date.now();
  if (cachedStatistics && now - cachedAt < STATISTICS_TTL_MS) {
    return cachedStatistics;
  }

  const result = await fetchStatistics(fetchFn);

  if (result.ok) {
    cachedStatistics = result.data;
    cachedAt = now;
    return result.data;
  }

  console.error(result.message);
  return cachedStatistics ?? EMPTY_STATISTICS;
}

export const load: PageLoad = async ({ fetch, url }) => {
  const searchRequest = parseSearchRequestFromParams(url.searchParams);
  const [searchResult, statisticsResponse] = await Promise.all([
    searchContracts(searchRequest, fetch),
    getCachedStatistics(fetch),
  ]);

  if (!searchResult.ok) {
    const rateLimited = searchResult.status === 429;
    return {
      searchResponse: EMPTY_SEARCH_RESPONSE,
      searchRequest,
      statisticsResponse,
      rateLimited,
      error: rateLimited ? null : searchResult.message,
    };
  }

  return {
    searchResponse: searchResult.data,
    searchRequest,
    statisticsResponse,
    rateLimited: false,
    error: null,
  };
};

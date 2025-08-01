import type {
  SearchContractsRequest,
  SearchContractsResponse,
} from "$lib/types/api";

export const DEFAULT_SEARCH_REQUEST: SearchContractsRequest = {
  query: "",
  sort: {
    field: "publicationDate",
    direction: "descending",
  },
  page: 1,
};

export async function searchContracts(
  data: SearchContractsRequest,
  fetchFn = fetch,
): Promise<SearchContractsResponse> {
  const response = await fetchFn(`/api/search`, {
    method: "POST",
    body: JSON.stringify(data),
    headers: { "Content-Type": "application/json" },
  });

  return await response.json();
}

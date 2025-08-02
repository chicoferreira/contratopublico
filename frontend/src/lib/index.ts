import type { SearchContractsRequest, SearchContractsResponse } from "$lib/types/api";

export const DEFAULT_SEARCH_REQUEST = {
  query: "",
  sort: {
    field: "publicationDate" as const,
    direction: "descending" as const,
  },
  page: 1,
} as const;

export async function searchContracts(
  data: SearchContractsRequest,
  fetchFn = fetch,
  signal?: AbortSignal,
): Promise<SearchContractsResponse> {
  const response = await fetchFn(`/api/search`, {
    method: "POST",
    body: JSON.stringify(data),
    headers: { "Content-Type": "application/json" },
    signal,
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({ message: "Unknown error occurred" }));
    throw new Error(errorData.message || `Error ${response.status}: ${response.statusText}`);
  }

  return await response.json();
}

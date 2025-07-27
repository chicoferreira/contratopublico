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
  baseURL: string = "",
): Promise<SearchContractsResponse> {
  const params = new URLSearchParams();

  params.append("query", data.query);
  if (data.filter) params.append("filter", data.filter);
  if (data.sort) {
    params.append("sortField", data.sort.field);
    params.append("sortDirection", data.sort.direction);
  }
  if (data.page) params.append("page", data.page.toString());

  const response = await fetch(`${baseURL}/api/search?${params}`, {
    method: "GET",
    headers: { "Content-Type": "application/json" },
  });

  return await response.json();
}

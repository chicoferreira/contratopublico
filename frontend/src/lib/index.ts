import type {
  Contract,
  SearchContractsRequest,
  SearchContractsResponse,
} from "$lib/types/api";

export async function searchContracts(
  data: SearchContractsRequest,
): Promise<SearchContractsResponse> {
  const params = new URLSearchParams();

  params.append("query", data.query);
  if (data.filter) params.append("filter", data.filter);
  if (data.sort) params.append("sort", data.sort.join(","));
  if (data.page) params.append("page", data.page.toString());
  if (data.offset) params.append("offset", data.offset.toString());

  const response = await fetch(`http://localhost:3000/search?${params}`, {
    method: "GET",
    headers: { "Content-Type": "application/json" },
  });

  return await response.json();
}

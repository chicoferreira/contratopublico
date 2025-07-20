import type {
  SearchContractsRequest,
  SearchContractsResponse,
} from "$lib/types/api";
import { env } from "$env/dynamic/private";

const BACKEND_URL = env.BACKEND_URL || "http://localhost:3000";

export async function searchContracts(
  data: SearchContractsRequest,
): Promise<SearchContractsResponse> {
  const params = new URLSearchParams();

  params.append("query", data.query);
  if (data.filter) params.append("filter", data.filter);
  if (data.sort) params.append("sort", data.sort.join(","));
  if (data.page) params.append("page", data.page.toString());
  if (data.offset) params.append("offset", data.offset.toString());

  const response = await fetch(`${BACKEND_URL}/search?${params}`, {
    method: "GET",
    headers: { "Content-Type": "application/json" },
  });

  return await response.json();
}

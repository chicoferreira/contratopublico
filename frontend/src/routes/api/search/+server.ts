import { json } from "@sveltejs/kit";
import { searchContracts } from "$lib/index";
import type { SearchContractsRequest } from "$lib/types/api";
import type { RequestHandler } from "./$types";

// TODO: contact backend directly

export const GET: RequestHandler = async ({ url }) => {
  const query = url.searchParams.get("query") || "";
  const filter = url.searchParams.get("filter") || null;
  const sortField = url.searchParams.get("sort[field]");
  const sortDirection = url.searchParams.get("sort[direction]");
  const sort =
    sortField && sortDirection
      ? { field: sortField as any, direction: sortDirection as any }
      : null;
  const page = url.searchParams.get("page")
    ? parseInt(url.searchParams.get("page")!)
    : null;
  const offset = url.searchParams.get("offset")
    ? parseInt(url.searchParams.get("offset")!)
    : null;

  const request: SearchContractsRequest = {
    query,
    filter,
    sort,
    page,
    offset,
  };

  console.log("Search request:", request);

  try {
    const response = await searchContracts(request);
    return json(response);
  } catch (error) {
    console.error("Search error:", error);
    return json({ error: "Failed to search contracts" }, { status: 500 });
  }
};

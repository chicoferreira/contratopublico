import { searchContracts } from "$lib/index";
import type { SearchContractsRequest, Sort } from "$lib/types/api";
import type { PageServerLoad } from "./$types";
import { env } from "$env/dynamic/private";

export const load: PageServerLoad = async () => {
  const defaultSort: Sort.SortBy = {
    field: "publicationDate",
    direction: "descending",
  };

  const defaultPage = 1;

  const defaultRequest: SearchContractsRequest = {
    query: "",
    sort: defaultSort,
    page: defaultPage,
  };

  const backendURL = env.BACKEND_URL || "http://localhost:3000";

  return {
    contracts: await searchContracts(defaultRequest, backendURL),
    sort: defaultSort,
    page: defaultPage,
  };
};

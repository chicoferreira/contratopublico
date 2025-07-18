import { searchContracts } from "$lib/index";
import type { SearchContractsRequest } from "$lib/types/api";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async () => {
  const defaultRequest: SearchContractsRequest = {
    query: "",
  };
  return await searchContracts(defaultRequest);
};

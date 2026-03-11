import type { PageLoad } from "./$types";
import { getContract } from "$lib";

export const load: PageLoad = async ({ params, fetch }) => {
  const contractId = Number(params.id);

  if (Number.isNaN(contractId)) {
    return { contract: undefined, rateLimited: false, error: null };
  }

  const result = await getContract(contractId, fetch);

  if (!result.ok) {
    const rateLimited = result.status === 429;
    return {
      contract: undefined,
      rateLimited,
      error: rateLimited ? null : result.message,
    };
  }

  return { contract: result.data, rateLimited: false, error: null };
};

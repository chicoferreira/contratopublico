import type { PageLoad } from "./$types";
import { getContract } from "$lib";

export const load: PageLoad = async ({ params, fetch }) => {
  const contractId = Number(params.id);

  if (Number.isNaN(contractId)) {
    return { contract: undefined, error: null };
  }

  try {
    const contract = await getContract(contractId, fetch);
    return { contract, error: null };
  } catch (err) {
    console.error("Failed to load contract:", err);
    return {
      contract: undefined,
      error: err instanceof Error ? err.message : "Erro desconhecido",
    };
  }
};

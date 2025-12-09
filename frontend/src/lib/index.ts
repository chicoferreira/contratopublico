import {
  Sort,
  type Filters,
  type GetContractResponse,
  type SearchContractsRequest,
  type SearchContractsResponse,
  type Statistics,
} from "$lib/types/api";
import { validateEnumOrDefault } from "./utils";

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

  if (!response.ok || response.status !== 200) {
    const errorData = await response.json().catch(() => ({ message: "Unknown error occurred" }));
    throw new Error(errorData.message || `Error ${response.status}: ${response.statusText}`);
  }

  return await response.json();
}

export async function getContract(id: number, fetchFn = fetch): Promise<GetContractResponse> {
  const response = await fetchFn(`/api/contract/${id}`, {
    method: "GET",
    headers: { "Content-Type": "application/json" },
  });

  if (!response.ok || response.status !== 200) {
    const errorData = await response.json().catch(() => ({ message: "Unknown error occurred" }));
    throw new Error(errorData.message || `Error ${response.status}: ${response.statusText}`);
  }

  return await response.json();
}

export function parseSearchRequestFromParams(
  params: URLSearchParams,
): Required<SearchContractsRequest> {
  const query = params.get("query") || DEFAULT_SEARCH_REQUEST.query;
  const sortField = validateEnumOrDefault(
    params.get("sortField"),
    Sort.fields,
    DEFAULT_SEARCH_REQUEST.sort!.field,
  );
  const sortDirection = validateEnumOrDefault(
    params.get("sortDirection"),
    Sort.directions,
    DEFAULT_SEARCH_REQUEST.sort!.direction,
  );
  const pageParam = params.get("page");
  let page = pageParam ? parseInt(pageParam, 10) : DEFAULT_SEARCH_REQUEST.page!;
  page = Math.max(1, page);

  const sort: Sort.SortBy = { field: sortField, direction: sortDirection };

  let filters: Filters = {};

  const addParam = (key: keyof Filters, mapper: (value: string) => string | number = (v) => v) => {
    const value = params.get(key);
    if (value) {
      filters[key as keyof typeof filters] = mapper(value) as any;
    }
  };

  addParam("minId", (v) => parseInt(v, 10));
  addParam("maxId", (v) => parseInt(v, 10));
  addParam("contracted");
  addParam("contracting");
  addParam("startPublicationDate");
  addParam("endPublicationDate");
  addParam("startSigningDate");
  addParam("endSigningDate");
  addParam("minPrice", (v) => parseInt(v, 10));
  addParam("maxPrice", (v) => parseInt(v, 10));

  return { query, sort, filters, page };
}

export async function fetchStatistics(fetchFn = fetch): Promise<Statistics> {
  const response = await fetchFn(`/api/statistics`, {
    method: "GET",
    headers: { "Content-Type": "application/json" },
  });

  if (!response.ok || response.status !== 200) {
    const errorData = await response.json().catch(() => ({ message: "Unknown error occurred" }));
    throw new Error(errorData.message || `Error ${response.status}: ${response.statusText}`);
  }

  return await response.json();
}

export function getBaseGovContractUrl(contractId: number) {
  return `https://www.base.gov.pt/Base4/pt/detalhe/?type=contratos&id=${contractId}`;
}

export function getBaseGovDocumentUrl(documentId: number) {
  return `https://www.base.gov.pt/Base4/pt/resultados/?type=doc_documentos&id=${documentId}`;
}

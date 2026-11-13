import {
  Sort,
  type CpvList,
  type Filters,
  type GetContractResponse,
  type SearchContractsRequest,
  type SearchContractsResponse,
  type Statistics,
} from "$lib/types/api";
import { validateEnumOrDefault } from "./utils";
import { CONTRACT_TYPES } from "./contract-types";
import { PROCEDURE_TYPES } from "./procedure-types";

export type ApiResult<T> = { ok: true; data: T } | { ok: false; status: number; message: string };

async function apiFetch<T>(
  fetchFn: typeof fetch,
  input: string,
  init?: RequestInit,
): Promise<ApiResult<T>> {
  try {
    const response = await fetchFn(input, {
      headers: { "Content-Type": "application/json" },
      ...init,
    });

    if (!response.ok || response.status !== 200) {
      const errorData = await response.json().catch(() => ({ message: "Erro desconhecido" }));
      return {
        ok: false,
        status: response.status,
        message: errorData.message || `Error ${response.status}: ${response.statusText}`,
      };
    }

    return { ok: true, data: await response.json() };
  } catch (e) {
    return { ok: false, status: 0, message: e instanceof Error ? e.message : "Erro desconhecido" };
  }
}

export const MAX_QUERY_LENGTH = 512;
export const MAX_ENTITY_FILTER_LENGTH = 256;
export const MAX_CPV_FILTERS = 50;

const RECENT_FEATURE_DAYS = 30;

export function isRecentFeature(featureAddedDate: string) {
  return Date.now() < Date.parse(featureAddedDate) + RECENT_FEATURE_DAYS * 24 * 60 * 60 * 1000;
}

export const DEFAULT_SEARCH_REQUEST = {
  query: "",
  sort: {
    field: "publicationDate" as const,
    direction: "descending" as const,
  },
  page: 1,
} as const;

export function searchContracts(
  data: SearchContractsRequest,
  fetchFn = fetch,
  signal?: AbortSignal,
): Promise<ApiResult<SearchContractsResponse>> {
  return apiFetch(fetchFn, `/api/search`, {
    method: "POST",
    body: JSON.stringify(data),
    signal,
  });
}

export function getContract(id: number, fetchFn = fetch): Promise<ApiResult<GetContractResponse>> {
  return apiFetch(fetchFn, `/api/contract/${id}`);
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

  const addParam = (key: keyof Filters, mapper = (v: string): Filters[keyof Filters] => v) => {
    const value = params.get(key);
    const mapped = value ? mapper(value) : undefined;
    if (mapped !== undefined) {
      filters[key as keyof typeof filters] = mapped as any;
    }
  };

  const knownValues = <T extends string>(value: string, options: Record<T, unknown>) => {
    const selected = value.split(",");
    const values = (Object.keys(options) as T[]).filter((option) => selected.includes(option));
    return values.length ? values : undefined;
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
  addParam("contractTypes", (v) => knownValues(v, CONTRACT_TYPES));
  addParam("procedureTypes", (v) => knownValues(v, PROCEDURE_TYPES));
  addParam("cpvs", (v) => v.split(",").slice(0, MAX_CPV_FILTERS));

  return { query, sort, filters, page };
}

export function buildSearchParams(request: Required<SearchContractsRequest>): URLSearchParams {
  const params = new URLSearchParams();

  if (request.query) params.set("query", request.query);

  const defaultSort = DEFAULT_SEARCH_REQUEST.sort;
  if (
    request.sort.field !== defaultSort.field ||
    request.sort.direction !== defaultSort.direction
  ) {
    params.set("sortField", request.sort.field);
    params.set("sortDirection", request.sort.direction);
  }

  if (request.page > DEFAULT_SEARCH_REQUEST.page) {
    params.set("page", request.page.toString());
  }

  Object.entries(request.filters).forEach(([field, value]) => {
    if (value != null && value !== "") {
      params.set(field, `${value}`);
    }
  });

  return params;
}

export function fetchStatistics(fetchFn = fetch): Promise<ApiResult<Statistics>> {
  return apiFetch(fetchFn, `/api/statistics`);
}

export function fetchCpvs(fetchFn = fetch): Promise<ApiResult<CpvList>> {
  return apiFetch(fetchFn, `/api/cpvs`);
}

export function getBaseGovContractUrl(contractId: number) {
  return `https://www.base.gov.pt/Base4/pt/detalhe/?type=contratos&id=${contractId}`;
}

export function getBaseGovAnnouncementUrl(announcementId: number) {
  return `https://www.base.gov.pt/Base4/pt/detalhe/?type=anuncios&id=${announcementId}`;
}

export function getBaseGovDocumentUrl(documentId: number) {
  return `https://www.base.gov.pt/Base4/pt/resultados/?type=doc_documentos&id=${documentId}`;
}

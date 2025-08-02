export interface Contract {
  id: number;
  contractingProcedureType: string;
  publicationDate: string;
  signingDate: string | null;
  ccp: boolean;
  contracted: string;
  contracting: string;
  objectBriefDescription: string;
  initialContractualPrice: number;
}

export interface SearchContractsRequest {
  query: string;
  filter?: string | null;
  sort?: Sort.SortBy | null;
  page?: number | null;
}

export interface SearchContractsResponse {
  contracts: (Contract & MatchingRanges)[];
  total: number;
  page: number;
  totalPages: number;
  elapsedMillis: number;
  hitsPerPage: number;
}

export namespace Sort {
  export interface SortBy {
    field: Field;
    direction: Direction;
  }

  export const fields = ["id", "publicationDate", "signingDate", "price"] as const;
  export type Field = (typeof fields)[number];

  export const directions = ["ascending", "descending"] as const;
  export type Direction = (typeof directions)[number];
}

export interface MatchingRanges {
  matchingRanges: {
    [key: string]: MatchingRange[];
  };
}

export interface MatchingRange {
  start: number;
  end: number;
}

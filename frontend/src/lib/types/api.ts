export interface Contract {
  id: number;
  description: string | null;
  objectBriefDescription: string;
  contractingProcedureType: string;
  contracting: Entity[];
  contracted: Entity[];
  cpv: Cpv;
  publicationDate: string;
  signingDate: string | null;
  initialContractualPrice: number;
  regime: string;
  contractStatus: number | string;
  nonWrittenContractJustificationTypes: string;
  contractTypes: string;
  executionDeadlineDays: number;
  executionPlace: string;
  contractFundamentationType: string;
  contestants: Entity[];
  invitees: Entity[];
  documents: Document[];
  ccp: boolean;
}

export interface Entity {
  id: number;
  nif: string;
  description: string;
}

export interface Cpv {
  code: string;
  designation: string;
}

export interface Document {
  id: number;
  description: string;
}

export interface SearchContractsRequest {
  query: string;
  sort?: Sort.SortBy;
  filters?: Filters;
  page?: number;
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

export interface Filters {
  minId?: number;
  maxId?: number;
  startPublicationDate?: string;
  endPublicationDate?: string;
  startSigningDate?: string;
  endSigningDate?: string;
  contracted?: string;
  contracting?: string;
  minPrice?: number;
  maxPrice?: number;
}

export interface MatchingRanges {
  matchingRanges: {
    [key: string]: MatchingRange[];
  };
}

export interface MatchingRange {
  start: number;
  end: number;
  indices?: number[];
}

export interface Statistics {
  totalSpentLast365Days: number;
  contractsLast365Days: number;
  totalSpentLast30Days: number;
  contractsLast30Days: number;
  totalSpentLast7Days: number;
  contractsLast7Days: number;
}

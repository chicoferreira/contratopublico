export interface Contract {
  id: number;
  contractingProcedureType: string;
  publicationDate: string;
  signingDate: string;
  ccp: boolean;
  contracted: string;
  contracting: string;
  objectBriefDescription: string;
  initialContractualPrice: string;
}

export interface SearchContractsRequest {
  query: string;
  filter?: string | null;
  sort?: [string] | null;
  page?: number | null;
  offset?: number | null;
}

export interface SearchContractsResponse {
  contracts: Contract[];
  total: number | null;
  estimatedTotal: number | null;
  page: number;
  offset: number;
}

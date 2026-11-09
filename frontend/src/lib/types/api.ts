export interface Contract {
  id: number;
  description: string | null;
  objectBriefDescription: string;
  procedureType: ProcedureType | null;
  contracting: Entity[];
  contracted: Entity[];
  cpvs: Cpv[];
  publicationDate: string;
  signingDate: string | null;
  initialContractualPrice: number;
  regime: string | null;
  contractStatus: string | null;
  nonWrittenContractJustificationTypes: string[];
  contractTypes: ContractType[];
  executionDeadlineDays: number;
  executionPlaces: string[];
  contractFundamentationType: string;
  contestants: Entity[];
  invitees: Entity[];
  documents: Document[];
  contractingProcedureUrl: string | null;
  announcementId: number | null;
  directAwardFundamentationType: string | null;
  observations: string | null;
  endOfContractType: string | null;
  closeDate: string | null;
  totalEffectivePrice: number | null;
  causesDeadlineChange: string | null;
  causesPriceChange: string | null;
  estimatedBasePrice: number | null;
  awardDecisionDate: string | null;
  frameworkAgreementDescription: string | null;
  specialMeasures: string | null;
  ambientCriteria: boolean;
}

export type ProcedureType =
  | "directAward"
  | "directAwardRuralFires"
  | "simplifiedDirectAward"
  | "simplifiedDirectAwardSpecialMeasures"
  | "priorConsultation"
  | "priorConsultationRuralFires"
  | "simplifiedPriorConsultation"
  | "publicTender"
  | "simplifiedPublicTender"
  | "restrictedTender"
  | "simplifiedRestrictedTender"
  | "negotiatedProcedure"
  | "competitiveDialogue"
  | "innovationPartnership"
  | "frameworkAgreementArt258"
  | "frameworkAgreementArt259"
  | "goodsProvision"
  | "socialAndOtherSpecificServices"
  | "simplifiedDesignContest"
  | "simplifiedIdeasContest"
  | "specialSectorsExemption"
  | "excludedContracting";

export const CONTRACT_TYPE_LABELS = {
  goodsAcquisition: "Aquisição de Bens Móveis",
  servicesAcquisition: "Aquisição de Serviços",
  publicWorksConcession: "Concessão de Obras Públicas",
  publicServicesConcession: "Concessão de Serviços Públicos",
  publicWorks: "Empreitadas de Obras Públicas",
  goodsLease: "Locação de Bens Móveis",
  partnership: "Sociedade",
  other: "Outros",
} as const;

export type ContractType = keyof typeof CONTRACT_TYPE_LABELS;

export const CONTRACT_TYPES = Object.keys(CONTRACT_TYPE_LABELS) as ContractType[];

export type ContractTypeLabel = (typeof CONTRACT_TYPE_LABELS)[ContractType];

export const CONTRACT_TYPE_BY_LABEL = new Map<ContractTypeLabel, ContractType>(
  Object.entries(CONTRACT_TYPE_LABELS).map(([type, label]) => [label, type as ContractType]),
);

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
  contracts: SearchedContract[];
  total: number;
  page: number;
  totalPages: number;
  elapsedMillis: number;
  hitsPerPage: number;
}

export interface SearchableContract {
  id: number;
  procedureType: ProcedureType | null;
  procedureTypeTitle: string | null;
  procedureTypeSubtitle: string | null;
  publicationDate: string;
  signingDate: string | null;
  objectBriefDescription: string;
  initialContractualPrice: number;
  contracting: Entity[];
  contracted: Entity[];
  cpvs: Cpv[];
  regime: string | null;
  contractTypeLabels: ContractTypeLabel[];
  executionPlaces: string[];
  contractFundamentationType: string;
  contestants: Entity[];
  invitees: Entity[];
  documents: Document[];
  contractingProcedureUrl: string | null;
  announcementId: number | null;
}

export type SearchedContract = SearchableContract & MatchingRanges;

export type GetContractResponse = Contract | null;

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
  contractTypes?: ContractType[];
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

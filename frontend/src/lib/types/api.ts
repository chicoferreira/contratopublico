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

export const PROCEDURE_TYPE_LABELS = {
  directAward: "Ajuste Direto Regime Geral",
  directAwardRuralFires:
    "Ajuste Direto Regime Geral ao abrigo do artigo 7º da Lei n.º 30/2021, de 21.05",
  simplifiedDirectAward: "Ajuste direto simplificado",
  simplifiedDirectAwardSpecialMeasures:
    "Ajuste direto simplificado ao abrigo da Lei n.º 30/2021, de 21.05",
  priorConsultation: "Consulta Prévia",
  priorConsultationRuralFires:
    "Consulta prévia ao abrigo do artigo 7º da Lei n.º 30/2021, de 21.05",
  simplifiedPriorConsultation: "Consulta Prévia Simplificada",
  publicTender: "Concurso público",
  simplifiedPublicTender: "Concurso público simplificado",
  restrictedTender: "Concurso limitado por prévia qualificação",
  simplifiedRestrictedTender: "Concurso limitado por prévia qualificação simplificado",
  negotiatedProcedure: "Procedimento de negociação",
  competitiveDialogue: "Diálogo concorrencial",
  innovationPartnership: "Parceria para a inovação",
  frameworkAgreementArt258: "Ao abrigo de acordo-quadro (art.º 258.º)",
  frameworkAgreementArt259: "Ao abrigo de acordo-quadro (art.º 259.º)",
  goodsProvision: "Disponibilização de bens móveis",
  socialAndOtherSpecificServices: "Serviços sociais e outros serviços específicos",
  simplifiedDesignContest: "Concurso de conceção simplificado",
  simplifiedIdeasContest: "Concurso de ideias simplificado",
  specialSectorsExemption: "Setores especiais – isenção parte II",
  excludedContracting: "Contratação excluída II",
} as const;

export type ProcedureType = keyof typeof PROCEDURE_TYPE_LABELS;

export type ProcedureTypeLabel = (typeof PROCEDURE_TYPE_LABELS)[ProcedureType];

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
  procedureTypeLabel: ProcedureTypeLabel | null;
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

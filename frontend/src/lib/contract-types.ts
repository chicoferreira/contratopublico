import type { ContractType } from "$lib/types/api";

export const CONTRACT_TYPES: Record<
  ContractType,
  { label: string; description: string; article?: string }
> = {
  goodsAcquisition: {
    label: "Aquisição de Bens Móveis",
    description: "A entidade pública compra bens móveis a um fornecedor.",
    article: "artigo 437.º",
  },
  servicesAcquisition: {
    label: "Aquisição de Serviços",
    description:
      "A entidade pública adquire a prestação de um ou vários tipos de serviços mediante o pagamento de um preço.",
    article: "artigo 450.º",
  },
  publicWorksConcession: {
    label: "Concessão de Obras Públicas",
    description:
      "Um parceiro executa uma obra pública e, em troca, fica com o direito de a explorar, assumindo o risco.",
    article: "artigo 407.º",
  },
  publicServicesConcession: {
    label: "Concessão de Serviços Públicos",
    description:
      "Um parceiro gere um serviço público e, em troca, fica com o direito de o explorar, assumindo o risco.",
    article: "artigo 407.º",
  },
  publicWorks: {
    label: "Empreitadas de Obras Públicas",
    description:
      "Execução, ou conceção e execução, de uma obra pública, como a construção, reabilitação, conservação ou demolição de um imóvel.",
    article: "artigo 343.º",
  },
  goodsLease: {
    label: "Locação de Bens Móveis",
    description:
      "Um locador proporciona à entidade pública o uso temporário de bens móveis, mediante retribuição.",
    article: "artigo 431.º",
  },
  partnership: {
    label: "Sociedade",
    description:
      "A entidade pública cria uma empresa com outros parceiros. Nos dados do Portal BASE, este tipo surge sobretudo em compras comuns.",
  },
  other: {
    label: "Outros",
    description: "Contrato que não se enquadra em nenhum dos outros tipos.",
  },
};

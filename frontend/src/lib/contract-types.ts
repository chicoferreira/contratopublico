import type { ContractType } from "$lib/types/api";

export const CONTRACT_TYPE_DESCRIPTIONS: Record<ContractType, string> = {
  goodsAcquisition: "A entidade pública compra bens móveis a um fornecedor.",
  servicesAcquisition:
    "A entidade pública adquire a prestação de um ou vários tipos de serviços mediante o pagamento de um preço.",
  publicWorksConcession:
    "Um parceiro executa uma obra pública e, em troca, fica com o direito de a explorar, assumindo o risco.",
  publicServicesConcession:
    "Um parceiro gere um serviço público e, em troca, fica com o direito de o explorar, assumindo o risco.",
  publicWorks:
    "Execução, ou conceção e execução, de uma obra pública, como a construção, reabilitação, conservação ou demolição de um imóvel.",
  goodsLease:
    "Um locador proporciona à entidade pública o uso temporário de bens móveis, mediante retribuição.",
  partnership:
    "A entidade pública cria uma empresa com outros parceiros. Nos dados do Portal BASE, este tipo surge sobretudo em compras comuns.",
  other: "Contrato que não se enquadra em nenhum dos outros tipos.",
};

export const CONTRACT_TYPE_ARTICLES: Partial<Record<ContractType, string>> = {
  goodsAcquisition: "artigo 437.º",
  servicesAcquisition: "artigo 450.º",
  publicWorksConcession: "artigo 407.º",
  publicServicesConcession: "artigo 407.º",
  publicWorks: "artigo 343.º",
  goodsLease: "artigo 431.º",
};

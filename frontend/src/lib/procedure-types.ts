import type { ProcedureType } from "$lib/types/api";

export type Competition = "open" | "invited" | "direct" | "none";

export const COMPETITION: Record<Competition, { label: string; dot: string }> = {
  open: {
    label: "Anúncio público",
    dot: "bg-green-500",
  },
  invited: {
    label: "Convite a várias entidades",
    dot: "bg-amber-500",
  },
  direct: {
    label: "Convite a uma entidade",
    dot: "bg-red-500",
  },
  none: {
    label: "Sem procedimento de concorrência",
    dot: "bg-gray-400",
  },
};

const SPECIAL_MEASURES =
  "permitido pelas medidas especiais de contratação pública (projetos com fundos europeus, habitação, tecnologias de informação, saúde, entre outros)";

const RURAL_FIRES =
  "usado por entidades do Sistema de Gestão Integrada de Fogos Rurais em contratos de gestão de combustíveis (vegetação).";

const CCP_URL =
  "https://www.base.gov.pt/Base4/pt/documentacao/codigo-dos-contratos-publicos-consolidado/";
const SPECIAL_MEASURES_URL = "https://diariodarepublica.pt/dr/detalhe/lei/30-2021-163728569";

const ccp = (article: string) => ({
  text: `artigo ${article} do Código dos Contratos Públicos`,
  url: CCP_URL,
});
const specialMeasures = (article: string) => ({
  text: `artigo ${article} da Lei n.º 30/2021`,
  url: SPECIAL_MEASURES_URL,
});

export const PROCEDURE_TYPES: Record<
  ProcedureType,
  {
    title: string;
    subtitle?: string;
    baseName?: string;
    competition: Competition;
    description: string;
    usage?: string;
    legalBasis?: { text: string; url: string };
  }
> = {
  directAward: {
    title: "Ajuste Direto",
    baseName: "Ajuste Direto Regime Geral",
    competition: "direct",
    description:
      "A entidade pública convida diretamente uma única entidade à sua escolha a apresentar proposta.",
    usage:
      "Permitido abaixo de 75 000 € em bens e serviços ou de 150 000 € em obras. Acima disso, só nos casos previstos na lei, como urgência imperiosa, um único fornecedor possível ou um concurso anterior sem propostas válidas.",
    legalBasis: ccp("112.º"),
  },
  directAwardRuralFires: {
    title: "Ajuste Direto",
    subtitle: "Fogos Rurais",
    baseName: "Ajuste Direto Regime Geral ao abrigo do artigo 7º da Lei n.º 30/2021, de 21.05",
    competition: "direct",
    description: `Ajuste direto ${RURAL_FIRES}`,
    usage: "Permitido abaixo dos limiares europeus e de 750 000 €.",
    legalBasis: specialMeasures("7.º"),
  },
  simplifiedDirectAward: {
    title: "Ajuste Direto",
    subtitle: "Simplificado",
    competition: "direct",
    description:
      "Ajuste direto para compras de pequeno valor, adjudicado diretamente sobre a fatura e sem outras formalidades.",
    usage: "Permitido até 5 000 € em bens e serviços ou 10 000 € em obras.",
    legalBasis: ccp("128.º"),
  },
  simplifiedDirectAwardSpecialMeasures: {
    title: "Ajuste Direto",
    subtitle: "Simplificado Especial",
    baseName: "Ajuste direto simplificado ao abrigo da Lei n.º 30/2021, de 21.05",
    competition: "direct",
    description: `Ajuste direto simplificado com um limite de valor mais alto, ${SPECIAL_MEASURES}.`,
    usage: "Permitido até 15 000 €.",
    legalBasis: specialMeasures("2.º"),
  },
  priorConsultation: {
    title: "Consulta Prévia",
    competition: "invited",
    description:
      "A entidade pública convida diretamente pelo menos três entidades à sua escolha a apresentar proposta, podendo negociar com elas se o convite o previr.",
    usage: "Permitido abaixo de 130 000 € em bens e serviços ou de 1 000 000 € em obras.",
    legalBasis: ccp("112.º"),
  },
  priorConsultationRuralFires: {
    title: "Consulta Prévia",
    subtitle: "Fogos Rurais",
    baseName: "Consulta prévia ao abrigo do artigo 7º da Lei n.º 30/2021, de 21.05",
    competition: "invited",
    description: `Consulta prévia ${RURAL_FIRES}`,
    usage: "Permitido abaixo dos limiares europeus e de 750 000 €.",
    legalBasis: specialMeasures("7.º"),
  },
  simplifiedPriorConsultation: {
    title: "Consulta Prévia",
    subtitle: "Simplificada",
    competition: "invited",
    description: `Consulta prévia com convite a pelo menos cinco entidades e prazos mais curtos, ${SPECIAL_MEASURES}.`,
    usage: "Permitido abaixo dos limiares europeus e de 750 000 €.",
    legalBasis: specialMeasures("2.º"),
  },
  publicTender: {
    title: "Concurso Público",
    competition: "open",
    description:
      "Procedimento anunciado no Diário da República, e no Jornal Oficial da União Europeia acima dos limiares europeus, em que qualquer interessado pode apresentar proposta.",
    usage: "Permitido em contratos de qualquer valor.",
    legalBasis: ccp("130.º"),
  },
  simplifiedPublicTender: {
    title: "Concurso Público",
    subtitle: "Simplificado",
    competition: "open",
    description: `Concurso público com prazos mais curtos, ${SPECIAL_MEASURES}.`,
    usage: "Permitido abaixo dos limiares europeus.",
    legalBasis: specialMeasures("2.º"),
  },
  restrictedTender: {
    title: "Concurso Limitado por Prévia Qualificação",
    competition: "open",
    description:
      "Procedimento anunciado publicamente em duas fases: primeiro os interessados candidatam-se e são qualificados, depois só os qualificados apresentam proposta.",
    usage: "Permitido em contratos de qualquer valor.",
    legalBasis: ccp("162.º"),
  },
  simplifiedRestrictedTender: {
    title: "Concurso Limitado por Prévia Qualificação",
    subtitle: "Simplificado",
    competition: "open",
    description: `Concurso limitado por prévia qualificação com prazos mais curtos, ${SPECIAL_MEASURES}.`,
    usage: "Permitido abaixo dos limiares europeus.",
    legalBasis: specialMeasures("2.º"),
  },
  negotiatedProcedure: {
    title: "Procedimento de Negociação",
    competition: "open",
    description:
      "Procedimento anunciado publicamente, com uma fase de qualificação, em que os concorrentes qualificados podem melhorar as propostas numa fase de negociação.",
    usage:
      "Só quando as necessidades não se satisfazem com soluções disponíveis no mercado, incluem soluções inovadoras, não é possível adjudicar sem negociar ou um concurso anterior não teve propostas aceitáveis.",
    legalBasis: ccp("193.º"),
  },
  competitiveDialogue: {
    title: "Diálogo Concorrencial",
    competition: "open",
    description:
      "Usado quando a entidade pública sabe o que precisa mas não como o obter: discute soluções com os candidatos qualificados antes de estes apresentarem propostas.",
    usage:
      "Só quando as necessidades não se satisfazem com soluções disponíveis no mercado, incluem soluções inovadoras, não é possível adjudicar sem negociar ou um concurso anterior não teve propostas aceitáveis.",
    legalBasis: ccp("204.º"),
  },
  innovationPartnership: {
    title: "Parceria para a Inovação",
    competition: "open",
    description:
      "Para investigar e desenvolver bens, serviços ou obras inovadores, que a entidade pública depois adquire se cumprirem os níveis de desempenho e os preços acordados.",
    legalBasis: ccp("218.º-A"),
  },
  frameworkAgreementArt258: {
    title: "Acordo-Quadro",
    subtitle: "Sem Nova Concorrência",
    baseName: "Ao abrigo de acordo-quadro (art.º 258.º)",
    competition: "direct",
    description:
      "Compra feita ao abrigo de um contrato anterior, celebrado com um ou mais fornecedores, que já fixou todas as condições (preços, prazos, especificações). A entidade pública encomenda diretamente, sem pedir novas propostas.",
    legalBasis: ccp("258.º"),
  },
  frameworkAgreementArt259: {
    title: "Acordo-Quadro",
    subtitle: "Com Nova Concorrência",
    baseName: "Ao abrigo de acordo-quadro (art.º 259.º)",
    competition: "invited",
    description:
      "Compra feita ao abrigo de um contrato anterior, celebrado com vários fornecedores, que deixou condições por fixar. A entidade pública pede propostas a esses fornecedores e escolhe a melhor.",
    legalBasis: ccp("259.º"),
  },
  goodsProvision: {
    title: "Disponibilização de Bens Móveis",
    competition: "none",
    description:
      "Disponibilização de bens móveis de uma entidade pública, comunicada ao Portal BASE.",
  },
  socialAndOtherSpecificServices: {
    title: "Serviços Sociais e Outros Serviços Específicos",
    competition: "open",
    description:
      "Aquisição de serviços sociais, de saúde, educação, cultura e outros serviços específicos, com regras próprias e anúncio no Jornal Oficial da União Europeia.",
    usage:
      "Aplica-se a contratos de valor igual ou superior a 750 000 €. Abaixo disso, estes serviços ficam fora das regras de formação de contratos do Código dos Contratos Públicos.",
    legalBasis: ccp("250.º-A"),
  },
  simplifiedDesignContest: {
    title: "Concurso de Conceção",
    subtitle: "Simplificado",
    competition: "open",
    description:
      "Concurso para selecionar trabalhos de conceção, como projetos de arquitetura ou engenharia, publicitado, no mínimo, no sítio da entidade pública.",
    usage: "Permitido quando os prémios e o eventual contrato seguinte não ultrapassam 75 000 €.",
    legalBasis: ccp("219.º-A"),
  },
  simplifiedIdeasContest: {
    title: "Concurso de Ideias",
    subtitle: "Simplificado",
    competition: "open",
    description:
      "Concurso para adquirir propostas de ideias, remuneradas com um prémio, segundo as regras do concurso de conceção simplificado.",
    legalBasis: ccp("219.º-J"),
  },
  specialSectorsExemption: {
    title: "Setores Especiais",
    subtitle: "Isenção",
    baseName: "Setores especiais – isenção parte II",
    competition: "none",
    description:
      "Contrato de uma entidade dos setores especiais (água, energia, transportes e serviços postais) isento das regras de formação de contratos do Código dos Contratos Públicos.",
  },
  excludedContracting: {
    title: "Contratação Excluída",
    baseName: "Contratação excluída II",
    competition: "none",
    description:
      "Contrato isento das regras de formação de contratos do Código dos Contratos Públicos, por exemplo por não estar sujeito à concorrência de mercado.",
    legalBasis: ccp("5.º"),
  },
};

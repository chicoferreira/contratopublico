export type ChangelogRef = { pr: number } | { issue: number } | { commit: string };

export interface ChangelogEntry {
  date: string;
  html: string;
  refs: ChangelogRef[];
}

export const CHANGELOG: ChangelogEntry[] = [
  {
    date: "2026-11-13",
    html: "É agora possível filtrar os contratos por código CPV, pesquisando pela designação ou pelo código numa árvore com todos os CPVs.",
    refs: [{ issue: 47 }, { pr: 83 }],
  },
  {
    date: "2026-11-13",
    html: "A explicação do CPV na página de pesquisa passou a mostrar a hierarquia completa do código.",
    refs: [{ pr: 83 }],
  },
  {
    date: "2026-11-09",
    html: "É agora possível filtrar os contratos por tipo de procedimento ou por nível de concorrência.",
    refs: [{ pr: 81 }],
  },
  {
    date: "2026-11-09",
    html: "A etiqueta do tipo de procedimento foi redesenhada para mostrar o nível de concorrência e agora todos os tipos de procedimento têm uma explicação.",
    refs: [{ pr: 81 }],
  },
  {
    date: "2026-11-07",
    html: "Os tipos de contrato agora aparecem como etiquetas na página de pesquisa e é possível filtrar por eles.",
    refs: [{ issue: 32 }, { pr: 80 }],
  },
  {
    date: "2026-11-05",
    html: "Os contratos com valor negativo passaram a ser tratados corretamente como um valor recebido pela entidade contratante.",
    refs: [{ commit: "e780e4a10543571a3ea58b85d49428207d47c670" }],
  },
  {
    date: "2026-11-05",
    html: "A página de cada contrato passou a mostrar mais informação, como o preço base e o estado do contrato.",
    refs: [{ commit: "0a0e15526a9dcce57605d63fd816b48f5c7d4938" }],
  },
  {
    date: "2026-11-03",
    html: "Os contratos que deixam de existir no Portal BASE e na API BASE passaram a ser removidos automaticamente da plataforma.",
    refs: [{ commit: "adc316d7481fc8a6484adcf8d834a4e561af94a8" }],
  },
  {
    date: "2026-11-02",
    html: 'Os contratos passaram a ser recolhidos também da <a href="https://www.base.gov.pt/APIBase2">API BASE</a>, além do Portal BASE, permitindo atualizar toda a base de dados muito mais rapidamente.',
    refs: [{ commit: "b4ddc75d061174b9474b0aee47e8f5492dd3a7a9" }],
  },
  {
    date: "2026-07-30",
    html: "Corrigido problema da barra de pesquisa que podia reverter para um texto escrito num momento anterior.",
    refs: [{ issue: 63 }, { commit: "f2f2ac36e6741cbb59d05d0e930b45ba27ec8950" }],
  },
  {
    date: "2025-12-09",
    html: "Cada contrato passou a ter uma página dedicada com todos os detalhes, acessível pelo seu título na página de pesquisa principal.",
    refs: [{ issue: 39 }, { commit: "f4366b7020958901479e394ec93dcb6639d9fc3e" }],
  },
  {
    date: "2025-12-09",
    html: "Na página de pesquisa, os contratos agora exibem a sua localização de execução.",
    refs: [{ issue: 51 }, { commit: "f4366b7020958901479e394ec93dcb6639d9fc3e" }],
  },
  {
    date: "2025-11-02",
    html: "Na página de pesquisa, os contratos agora exibem os códigos CPV e os NIFs das entidades contratantes e contratadas.",
    refs: [{ commit: "3cba8ea81087f4e015b6338b137eab51735d81c4" }],
  },
  {
    date: "2025-11-02",
    html: "O painel de filtros agora abre automaticamente quando há filtros ativos e pode ser fechado clicando no título.",
    refs: [{ issue: 37 }, { commit: "5dc6c6d0e50cb8dfe8ab2e0f86cf6384434fd53d" }],
  },
  {
    date: "2025-09-09",
    html: "Os contratos agora incluem campos como descrição, NIFs de contratados e contratantes, códigos CPV, documentos, e vários outros atributos",
    refs: [{ pr: 38 }],
  },
  {
    date: "2025-08-11",
    html: "Adicionado critério de desempate por ID do contrato na ordenação por data",
    refs: [{ commit: "f4a139d505bbf320f731d4852461552e33260302" }],
  },
  {
    date: "2025-08-10",
    html: "Esta página de changelog adicionada",
    refs: [{ commit: "d5386f779cb92d58c0b82d8a46ac8d92b586007d" }],
  },
  {
    date: "2025-08-10",
    html: "Na pesquisa de contratos, ao mudar de página no seletor inferior, a janela do navegador será movida para cima",
    refs: [{ commit: "2665c0e1bb1d7ed063894d900bfae5aa540fdece" }],
  },
  {
    date: "2025-08-08",
    html: 'Lançamento da Plataforma <a href="https://contratopublico.pt">contratopublico.pt</a>',
    refs: [],
  },
];

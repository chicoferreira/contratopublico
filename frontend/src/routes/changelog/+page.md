---
title: Changelog
description: Atualizações e melhorias da plataforma Contrato Público.
---

<script>
import Muted from "./Muted.svelte"
import MonthCommits from "./MonthCommits.svelte"
import GithubBadge from "./GithubBadge.svelte"
</script>

# Changelog da Plataforma

Este changelog documenta todas as alterações, melhorias e atualizações mais relevantes feitas na plataforma do **Contrato Público**.

## Julho de 2026 <MonthCommits startDate="2026-07-01" endDate="2026-07-31" />

- Corrigido problema da barra de pesquisa que podia reverter para um texto escrito num momento anterior. <GithubBadge issue="63"/> <GithubBadge commit="f2f2ac36e6741cbb59d05d0e930b45ba27ec8950"/> <Muted>(30/07/2026)</Muted>

## Dezembro de 2025 <MonthCommits startDate="2025-12-01" endDate="2025-12-31" />

- Cada contrato passou a ter uma página dedicada com todos os detalhes, acessível pelo seu título na página de pesquisa principal. <GithubBadge issue="39"/> <GithubBadge commit="f4366b7020958901479e394ec93dcb6639d9fc3e"/> <Muted>(09/12/2025)</Muted>
- Na página de pesquisa, os contratos agora exibem a sua localização de execução. <GithubBadge issue="51"/> <GithubBadge commit="f4366b7020958901479e394ec93dcb6639d9fc3e"/> <Muted>(09/12/2025)</Muted>

## Novembro de 2025 <MonthCommits startDate="2025-11-01" endDate="2025-11-30" />

- Na página de pesquisa, os contratos agora exibem os códigos CPV e os NIFs das entidades contratantes e contratadas. <GithubBadge commit="3cba8ea81087f4e015b6338b137eab51735d81c4"/> <Muted>(02/11/2025)</Muted>
- O painel de filtros agora abre automaticamente quando há filtros ativos e pode ser fechado clicando no título. <GithubBadge issue="37"/> <GithubBadge commit="5dc6c6d0e50cb8dfe8ab2e0f86cf6384434fd53d"/> <Muted>(02/11/2025)</Muted>

## Setembro de 2025 <MonthCommits startDate="2025-09-01" endDate="2025-09-30" />

- Os contratos agora incluem campos como descrição, NIFs de contratados e contratantes, códigos CPV, documentos, e vários outros atributos <GithubBadge pr="38"/> <Muted>(09/09/2025)</Muted>

## Agosto de 2025 <MonthCommits startDate="2025-08-01" endDate="2025-08-31" />

- Adicionado critério de desempate por ID do contrato na ordenação por data <GithubBadge commit="f4a139d505bbf320f731d4852461552e33260302"/> <Muted>(11/08/2025)</Muted>
- Esta página de changelog adicionada <GithubBadge commit="d5386f779cb92d58c0b82d8a46ac8d92b586007d"/> <Muted>(10/08/2025)</Muted>
- Na pesquisa de contratos, ao mudar de página no seletor inferior, a janela do navegador será movida para cima <GithubBadge commit="2665c0e1bb1d7ed063894d900bfae5aa540fdece"/> <Muted>(10/08/2025)</Muted>
- Lançamento da Plataforma [contratopublico.pt](https://contratopublico.pt) <Muted>(08/08/2025)</Muted>

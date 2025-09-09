---
title: Changelog
---

<script>
import Muted from "../../components/markdown/Muted.svelte"
import Commit from "../../components/markdown/Commit.svelte"
import MonthCommits from "../../components/markdown/MonthCommits.svelte"
import PullRequest from "../../components/markdown/PullRequest.svelte"
</script>

# Changelog da Plataforma

Este changelog documenta todas as alterações, melhorias e atualizações feitas na plataforma do **Contrato Público**.

## Setembro de 2025 <MonthCommits startDate="2025-09-01" endDate="2025-09-30" />

- Os contratos agora incluem campos como descrição, NIFs de contratados e contratantes, códigos CPV, documentos, e vários outros atributos <PullRequest pr="38"/> <Muted>(09/09/2025)</Muted>

## Agosto de 2025 <MonthCommits startDate="2025-08-01" endDate="2025-08-31" />

- Adicionado critério de desempate por ID do contrato na ordenação por data <Commit commit="f4a139d505bbf320f731d4852461552e33260302"/> <Muted>(11/08/2025)</Muted>
- Esta página de changelog adicionada <Commit commit="d5386f779cb92d58c0b82d8a46ac8d92b586007d"/> <Muted>(10/08/2025)</Muted>
- Na pesquisa de contratos, ao mudar de página no seletor inferior, a janela do navegador será movida para cima <Commit commit="2665c0e1bb1d7ed063894d900bfae5aa540fdece"/> <Muted>(10/08/2025)</Muted>
- Lançamento da Plataforma [contratopublico.pt](https://contratopublico.pt) <Muted>(08/08/2025)</Muted>

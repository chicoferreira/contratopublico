<script lang="ts">
  import Link from "../../../components/Link.svelte";
  import {
    BadgeEuro,
    Building,
    CalendarDays,
    ClipboardCheck,
    FileStack,
    FileText,
    MapPin,
    Signature,
    Tag,
    Users,
  } from "@lucide/svelte";
  import GridCard from "./GridCard.svelte";
  import GridCardTitle from "./GridCardTitle.svelte";
  import GridSubCard from "./GridSubCard.svelte";
  import GridSubCardList from "./GridSubCardList.svelte";
  import ContractProcedureTypeBadge from "../../../components/ContractProcedureTypeBadge.svelte";
  import GridCardTitleList from "./GridCardTitleList.svelte";
  import { dateToString, formatDate, formatMoney } from "$lib/utils";
  import { getBaseGovContractUrl, getBaseGovDocumentUrl } from "$lib";
  import ErrorDisplay from "../../../components/ErrorDisplay.svelte";

  const { data } = $props();
  const contract = data.contract;
  const error = data.error;

  const procedingUrlTitle = $derived.by(() => {
    if (!contract) return undefined;
    const url = contract.contractingProcedureUrl;
    if (!url || typeof url !== "string") return undefined;

    try {
      return new URL(url).hostname;
    } catch (error) {
      console.error("Invalid contracting procedure URL:", error);
      return undefined;
    }
  });

  const executionDeadlineDateString = $derived.by(() => {
    if (!contract) return undefined;
    if (!contract.executionDeadlineDays) return undefined;
    if (!contract.signingDate) return undefined;

    try {
      const date = new Date(contract.signingDate);
      date.setDate(date.getDate() + contract.executionDeadlineDays);
      return dateToString(date);
    } catch (error) {
      console.error("Invalid execution date:", error);
    }
  });
</script>

<svelte:head>
  <title>Contrato Público {contract ? `#${contract.id}` : ""}</title>
</svelte:head>

{#if error}
  <ErrorDisplay message={error} />
{:else if contract}
  <div class="space-y-4">
    <div class="space-y-1">
      <GridCard class="space-y-1.5">
        <div class="space-y-0.5">
          <div class="text-muted-foreground text-xs font-semibold tracking-wide uppercase">
            Contrato #{contract.id}
          </div>
          <div>
            <h1 class="text-2xl leading-snug font-semibold">{contract.objectBriefDescription}</h1>
            {#if contract.description && contract.description != contract.objectBriefDescription}
              <p class="text-muted-foreground">{contract.description}</p>
            {/if}
          </div>
        </div>
        <Link url={getBaseGovContractUrl(contract.id)}>Versão original no Portal BASE</Link>
      </GridCard>
    </div>

    <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      <GridCardTitle title="Valor contratual inicial" icon={BadgeEuro} compact={true}>
        <p class="text-lg font-semibold">{formatMoney(contract.initialContractualPrice)}</p>
        <p class="text-muted-foreground text-sm">Valor inicial estabelecido no contrato</p>
      </GridCardTitle>

      <GridCardTitle title="Data de publicação" icon={CalendarDays} compact={true}>
        <p class="text-lg font-semibold">{formatDate(contract.publicationDate)}</p>
        <p class="text-muted-foreground text-sm">Data de publicação no Portal BASE</p>
      </GridCardTitle>

      <GridCardTitle title="Data de assinatura" icon={Signature} compact={true}>
        <p class="text-lg font-semibold">{formatDate(contract.signingDate)}</p>
        <p class="text-muted-foreground text-sm">Data de formalização do contrato</p>
      </GridCardTitle>

      <GridCardTitle title="Local de execução" icon={MapPin} compact={true}>
        <p class="text-lg font-semibold">{contract.executionPlace || "—"}</p>
        <p class="text-muted-foreground text-sm">Área geográfica abrangida</p>
      </GridCardTitle>
    </div>

    <div class="grid gap-4 lg:grid-cols-2">
      <GridCardTitleList
        title={contract.contracting.length === 1
          ? "Entidade contratante"
          : "Entidades contratantes"}
        icon={FileText}
        elements={contract.contracting}
        fallback="Nenhuma entidade contratante">
        {#snippet renderElement(contracting)}
          <GridSubCard>
            <p class="leading-tight font-semibold">{contracting.description}</p>
            <p class="text-muted-foreground text-xs">NIF {contracting.nif}</p>
          </GridSubCard>
        {/snippet}
      </GridCardTitleList>

      <GridCardTitleList
        title={contract.contracted.length === 1 ? "Entidade contratada" : "Entidades contratadas"}
        icon={Building}
        elements={contract.contracted}
        fallback="Nenhuma entidade contratada">
        {#snippet renderElement(contracted)}
          <GridSubCard>
            <p class="leading-tight font-semibold">{contracted.description}</p>
            <p class="text-muted-foreground text-xs">NIF {contracted.nif}</p>
          </GridSubCard>
        {/snippet}
      </GridCardTitleList>
    </div>

    <div class="grid gap-4 lg:grid-cols-2">
      <GridCardTitle icon={FileText} title="Procedimento">
        <div class="space-y-2 text-sm">
          <p class="text-muted-foreground">
            Tipo de procedimento:
            <ContractProcedureTypeBadge type={contract.contractingProcedureType} />
          </p>
          <p class="text-muted-foreground">
            Tipo de contrato:
            <span class="text-foreground font-medium">{contract.contractTypes}</span>
          </p>
          <p class="text-muted-foreground">
            Regime:
            <span class="text-foreground font-medium">{contract.regime || "—"}</span>
          </p>
          <p class="text-muted-foreground">
            Fundamentação:
            <span class="text-foreground font-medium">{contract.contractFundamentationType}</span>
          </p>
          <p class="text-muted-foreground">
            Fundamentação (ajuste direto):
            <span class="text-foreground font-medium">
              {contract.directAwardFundamentationType}
            </span>
          </p>
          {#if contract.nonWrittenContractJustificationTypes}
            <p class="text-muted-foreground">
              Justificação não escrita:
              <span class="text-foreground font-medium">
                {contract.nonWrittenContractJustificationTypes}
              </span>
            </p>
          {/if}
          {#if contract.observations}
            <p class="text-muted-foreground">
              Observações:
              <span class="text-foreground font-medium">
                {contract.observations}
              </span>
            </p>
          {/if}
          <p class="text-muted-foreground">
            Peças do Procedimento:
            {#if contract.contractingProcedureUrl}
              <Link url={contract.contractingProcedureUrl} class="break-all">
                {procedingUrlTitle}
              </Link>
            {:else}
              <span class="text-foreground font-medium">Não disponível</span>
            {/if}
          </p>
        </div>
      </GridCardTitle>

      <GridCardTitle icon={ClipboardCheck} title="Execução">
        <div class="space-y-2 text-sm">
          <p class="text-muted-foreground">
            Prazo de execução:
            <span class="text-foreground font-medium">
              {contract.executionDeadlineDays} dias
              {#if executionDeadlineDateString}
                <span class="text-muted-foreground">({executionDeadlineDateString})</span>
              {/if}
            </span>
          </p>
          <p class="text-muted-foreground">
            Causa da extinção do contrato:
            <span class="text-foreground font-medium">
              {contract.endOfContractType || "—"}
            </span>
          </p>
          <p class="text-muted-foreground">
            Data do fecho de contrato:
            <span class="text-foreground font-medium">
              {formatDate(contract.closeDate)}
            </span>
          </p>
          <p class="text-muted-foreground">
            Preço total efetivo:
            <span class="text-foreground font-medium">
              {formatMoney(contract.totalEffectivePrice)}
            </span>
          </p>
          <p class="text-muted-foreground">
            Causas das alterações ao prazo:
            <span class="text-foreground font-medium">
              {contract.causesDeadlineChange || "—"}
            </span>
          </p>
          <p class="text-muted-foreground">
            Causas das alterações ao preço:
            <span class="text-foreground font-medium">
              {contract.causesPriceChange || "—"}
            </span>
          </p>
        </div>
      </GridCardTitle>
    </div>

    <div class="grid gap-4 sm:grid-cols-1 xl:grid-cols-2">
      <div class="grid gap-4">
        <GridCardTitleList
          title="Documentos"
          icon={FileStack}
          elements={contract.documents}
          fallback="Sem documentos associados">
          {#snippet renderElement(doc)}
            <GridSubCard class="text-sm">
              <Link url={getBaseGovDocumentUrl(doc.id)} class="font-medium break-all">
                {doc.description}
              </Link>
              <span class="text-muted-foreground ml-2">#{doc.id}</span>
            </GridSubCard>
          {/snippet}
        </GridCardTitleList>

        <GridCardTitleList
          title="CPVs"
          icon={Tag}
          fallback="Sem códigos CPV associados"
          elements={contract.cpvs}>
          {#snippet renderElement(cpv)}
            <GridSubCard class="text-sm">
              <span class="font-semibold">{cpv.code}</span>
              <span class="text-muted-foreground ml-2">{cpv.designation}</span>
            </GridSubCard>
          {/snippet}
        </GridCardTitleList>
      </div>

      <div class="grid gap-4">
        <GridCardTitle title="Outras entidades" icon={Users}>
          {#if contract.contestants.length > 0 || contract.invitees.length > 0}
            <div class="space-y-6">
              {#if contract.contestants.length > 0}
                <GridSubCardList
                  title="Concorrentes"
                  elements={contract.contestants}
                  fallback="Sem concorrentes associados">
                  {#snippet renderElement(elem)}
                    <GridSubCard>
                      <p class="leading-tight font-semibold">{elem.description}</p>
                      <p class="text-muted-foreground text-xs">NIF {elem.nif}</p>
                    </GridSubCard>
                  {/snippet}
                </GridSubCardList>
              {/if}

              {#if contract.invitees.length > 0}
                <GridSubCardList
                  title="Convidados"
                  elements={contract.invitees}
                  fallback="Sem convidados associados">
                  {#snippet renderElement(elem)}
                    <GridSubCard>
                      <p class="leading-tight font-semibold">{elem.description}</p>
                      <p class="text-muted-foreground text-xs">NIF {elem.nif}</p>
                    </GridSubCard>
                  {/snippet}
                </GridSubCardList>
              {/if}
            </div>
          {:else}
            <p class="text-muted-foreground text-sm">Sem outras entidades associadas</p>
          {/if}
        </GridCardTitle>
      </div>
    </div>
  </div>
{:else}
  <GridCard>
    <p class="text-muted-foreground text-sm">Não encontrámos o contrato pedido.</p>
  </GridCard>
{/if}

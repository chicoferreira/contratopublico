<script lang="ts">
  import ProsePage from "$lib/components/ProsePage.svelte";
  import Muted from "./Muted.svelte";
  import Link from "$lib/components/Link.svelte";
  import GithubBadge from "./GithubBadge.svelte";
  import { CHANGELOG } from "$lib/changelog";
  import { formatDate } from "$lib/utils";

  const monthFormatter = new Intl.DateTimeFormat("pt-PT", {
    month: "long",
    year: "numeric",
    timeZone: "UTC",
  });

  const months = Object.groupBy(CHANGELOG, (entry) => monthFormatter.format(new Date(entry.date)));

  const commitsUrl = (date: string) => {
    const d = new Date(date);
    const since = new Date(Date.UTC(d.getUTCFullYear(), d.getUTCMonth(), 1));
    const until = new Date(Date.UTC(d.getUTCFullYear(), d.getUTCMonth() + 1, 1));
    return `https://github.com/chicoferreira/contratopublico/commits?since=${since.toISOString()}&until=${until.toISOString()}`;
  };
</script>

<ProsePage title="Changelog" description="Atualizações e melhorias da plataforma Contrato Público.">
  <h1>Changelog da Plataforma</h1>

  <p>
    Este changelog documenta todas as alterações, melhorias e atualizações mais relevantes feitas na
    plataforma do <strong>Contrato Público</strong>.
  </p>

  {#each Object.entries(months) as [month, entries = []] (month)}
    <h2 class="first-letter:uppercase">
      {month}
      <Link url={commitsUrl(entries[0].date)}>Commits</Link>
    </h2>
    <ul>
      {#each entries as entry, i (i)}
        <li>
          {@html entry.html}
          {#each entry.refs as ref, j (j)}<GithubBadge {...ref} />{" "}{/each}
          <Muted>({formatDate(entry.date)})</Muted>
        </li>
      {/each}
    </ul>
  {/each}
</ProsePage>

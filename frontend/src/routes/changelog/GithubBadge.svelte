<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";

  type Props = { pr: number | string } | { issue: number | string } | { commit: string };

  const props: Props = $props();

  const pr = "pr" in props ? props.pr : undefined;
  const issue = "issue" in props ? props.issue : undefined;
  const commit = "commit" in props ? props.commit : undefined;

  const type = commit !== undefined ? "commit" : issue !== undefined ? "issue" : "pull";

  const segments = {
    commit: "commit",
    issue: "issues",
    pull: "pull",
  } as const;

  const badges = {
    commit: "bg-blue-500 dark:bg-blue-600",
    issue: "bg-green-500 dark:bg-green-600",
    pull: "bg-yellow-500 dark:bg-yellow-600",
  } as const;

  const id = String(pr ?? issue ?? commit);

  const url = `https://github.com/chicoferreira/contratopublico/${segments[type]}/${id}`;

  const label = type === "commit" ? id.slice(0, 7) : `#${id}`;

  const badgeClass = badges[type];
</script>

<Badge variant="secondary" class={badgeClass}>
  <a href={url} target="_blank" class="text-white hover:text-white/80">
    {label}
  </a>
</Badge>

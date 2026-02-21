<script lang="ts">
  import { AlertTriangle } from "@lucide/svelte";
  import Link from "./Link.svelte";
  import Separator from "$lib/components/ui/separator/separator.svelte";

  interface Props {
    message: string;
  }

  let { message }: Props = $props();

  const title = $derived(encodeURIComponent(`Error: ${message}`));
  const issueUrl = $derived(`https://github.com/chicoferreira/contratopublico/issues/new?title=${title}`);
</script>

<div
  class="border-destructive/20 bg-destructive/5 text-destructive space-y-3 rounded-lg border px-6 py-5">
  <div class="flex items-start gap-3">
    <AlertTriangle class="text-destructive h-8 w-8 shrink-0 self-center" />
    <div class="flex-1">
      <h3 class="text-sm font-semibold">Algo correu mal :(</h3>
      <p class="mt-1 text-sm">
        {message}
      </p>
    </div>
  </div>

  <div class="space-y-1 text-sm">
    <p>Pode tentar recarregar a página, pesquisar outro contrato ou aguardar alguns momentos.</p>
    <p>Se o erro persistir, a tua ajuda é importante para resolvermos o problema.</p>
  </div>

  <Separator class="bg-destructive/20" />

  <div class="space-y-2 text-sm">
    <p class="text-destructive font-medium">Como reportar este problema:</p>
    <ol class="marker:text-destructive/50 ml-6 list-outside list-decimal space-y-1">
      <li class="pl-2">
        Primeiro, tente recarregar a página <span class="text-destructive/60">(Ctrl+R ou F5)</span>
      </li>
      <li class="pl-2">
        Se o erro continuar, abra as Ferramentas de Programadores
        <span class="text-destructive/60">(F12 ou Ctrl+Shift+I)</span>
      </li>
      <li class="pl-2">Clique no separador "<em>Console</em>"</li>
      <li class="pl-2">Copie <strong>todos</strong> os logs de erros visíveis</li>
      <li class="pl-2">
        <Link
          url={issueUrl}
          class="text-destructive! hover:text-destructive/60! underline!"
          showIcon={false}>
          Crie um <em>issue</em> no GitHub
        </Link>
        e inclua na descrição:
        <ul class="mt-1 ml-4 list-inside list-disc space-y-0.5 text-xs">
          <li>Os logs de erro copiados</li>
          <li>O link da página onde o erro aconteceu</li>
          <li>
            O que fez para o erro acontecer
            <span class="text-destructive/60">(se aplicável)</span>
          </li>
          <li>
            O seu <em>browser</em>
            <span class="text-destructive/60">(ex: Chrome, Safari, Firefox)</span>
          </li>
        </ul>
      </li>
      <li class="pl-2">Submeta o <em>issue</em> para nos ajudar a corrigir o problema</li>
    </ol>
  </div>
</div>

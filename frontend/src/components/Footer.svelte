<script>
  import { Github, Bug, Lightbulb } from "@lucide/svelte";
  import Link from "./Link.svelte";
  import Separator from "$lib/components/ui/separator/separator.svelte";

  const currentYear = new Date().getFullYear();
</script>

<footer class="bg-muted/30 mt-auto border-y">
  <div class="container mx-auto px-4 py-7">
    <div class="flex flex-col items-center space-y-5 text-center">
      <div class="text-muted-foreground max-w-2xl space-y-2 text-sm">
        <p>
          Site não afiliado ao
          <span class="text-primary font-semibold">IMPIC</span>
          nem ao
          <span class="text-primary font-semibold">Governo de Portugal</span>
        </p>
        <p>
          Todas as informações são públicas e recolhidas do site
          <Link
            url="https://www.base.gov.pt"
            class="!text-primary hover:!text-primary font-medium">
            Portal BASE
          </Link>
        </p>
      </div>

      <div
        class="flex flex-col items-center gap-3 text-sm sm:flex-row sm:gap-4">
        <div class="inline-flex items-center gap-2">
          <Github class="text-muted-foreground h-4 w-4" />
          <Link
            url="https://github.com/chicoferreira/contratopublico"
            class="!text-muted-foreground hover:!text-primary !transition-colors"
            showIcon={false}>
            Ver código fonte
          </Link>
        </div>

        <Separator orientation="vertical" class="hidden !h-4 sm:block" />

        <div class="inline-flex items-center gap-2">
          <Bug class="text-muted-foreground h-4 w-4" />
          <Link
            url="https://github.com/chicoferreira/contratopublico/issues/new?template=bug_report.md"
            class="!text-muted-foreground hover:!text-primary !transition-colors"
            showIcon={false}>
            Reportar problema
          </Link>
        </div>

        <Separator orientation="vertical" class="hidden !h-4 sm:block" />

        <div class="inline-flex items-center gap-2">
          <Lightbulb class="text-muted-foreground h-4 w-4" />
          <Link
            url="https://github.com/chicoferreira/contratopublico/issues/new?template=feature_request.md"
            class="!text-muted-foreground hover:!text-primary !transition-colors"
            showIcon={false}>
            Sugerir funcionalidade
          </Link>
        </div>
      </div>

      <p class="text-muted-foreground text-sm">
        Desenvolvido por
        <Link
          url="https://github.com/chicoferreira"
          class="!text-primary hover:!text-primary !font-medium"
          showIcon={false}>
          chicoferreira
        </Link> © {currentYear}
      </p>
    </div>
  </div>
</footer>

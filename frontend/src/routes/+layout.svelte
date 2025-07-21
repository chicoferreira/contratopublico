<script>
  let { children } = $props();
  import "../app.css";

  import { Building2 } from "@lucide/svelte";
</script>

<div class="bg-primary text-primary-foreground">
  <div class="container mx-auto px-4 py-5">
    <div class="flex items-center gap-3">
      <Building2 class="h-8 w-8" />
      <div>
        <h1 class="text-3xl font-bold">Contrato Público</h1>
      </div>
    </div>
  </div>
</div>

<div class="bg-muted min-h-screen">
  <main class="container mx-auto">
    <div class="px-4 py-5">
      {@render children()}
    </div>
  </main>
</div>

<script lang="ts">
  import { Timer } from "@lucide/svelte";
</script>

<div
  class="border-amber-500/20 bg-amber-500/5 text-amber-900 dark:text-amber-200 space-y-3 rounded-lg border px-6 py-5">
  <div class="flex items-start gap-3">
    <Timer class="h-8 w-8 shrink-0 self-center text-amber-600 dark:text-amber-400" />
    <div class="flex-1">
      <h3 class="text-sm font-semibold">Calma, estás a fazer pedidos a mais!</h3>
      <p class="mt-1 text-sm text-amber-800 dark:text-amber-300">
        O servidor limitou temporariamente os teus pedidos para proteger o serviço. Aguarda um
        momento antes de tentar novamente.
      </p>
    </div>
  </div>
</div>

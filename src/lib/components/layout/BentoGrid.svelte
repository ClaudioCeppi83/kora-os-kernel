<script lang="ts">
  import { onMount } from "svelte";
  // Layout wrapper logic
</script>

<!-- Main Workspace Container -->
<main
  class="flex-1 flex gap-4 p-4 overflow-hidden h-full w-full bg-background-dark text-neutral-light font-display"
>
  <!-- Left Panel: System & Files -->
  <aside class="w-80 flex flex-col gap-4 overflow-y-auto shrink-0">
    <slot name="sidebar" />
  </aside>

  <!-- Center Panel: Main Terminal -->
  <section
    class="flex-1 flex flex-col bento-border bg-neutral-dark/20 rounded relative overflow-hidden min-w-0"
  >
    <slot name="main" />
  </section>
</main>

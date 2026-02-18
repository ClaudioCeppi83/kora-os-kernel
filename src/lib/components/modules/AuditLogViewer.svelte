<script lang="ts">
  import { bridge } from "$lib/bridge";
  import { onMount } from "svelte";
  import { Shield, X } from "lucide-svelte";

  export let isOpen = false;
  export let onToggle = () => {};

  let logs: any[] = [];
  let loading = false;

  async function fetchLogs() {
    loading = true;
    try {
      logs = await bridge.getAuditLogs();
    } catch (e) {
      console.error("Failed to fetch logs:", e);
    } finally {
      loading = false;
    }
  }

  $: if (isOpen) {
    fetchLogs();
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-[100] flex items-center justify-center p-8 bg-background-dark/80 backdrop-blur-md cursor-default"
    onclick={onToggle}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") onToggle();
    }}
    role="button"
    tabindex="0"
    aria-label="Close audit logs"
  >
    <div
      class="w-full max-w-4xl h-[70vh] bg-background-dark border border-primary/30 rounded-2xl flex flex-col overflow-hidden shadow-2xl relative"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
      role="presentation"
    >
      <!-- Header -->
      <div
        class="p-4 border-b border-primary/20 flex items-center justify-between bg-primary/5"
      >
        <div class="flex items-center gap-2">
          <Shield size={16} class="text-primary" />
          <h2
            class="text-sm font-mono font-bold uppercase tracking-widest text-primary"
          >
            Kernel Audit Logs
          </h2>
        </div>
        <button
          onclick={onToggle}
          class="text-neutral-light/50 hover:text-primary transition-colors p-2 -mr-2"
          aria-label="Close"
        >
          <X size={18} />
        </button>
      </div>

      <!-- Log List -->
      <div class="flex-1 overflow-y-auto p-4 font-mono text-[10px]">
        {#if loading}
          <div
            class="flex items-center justify-center h-full text-primary/50 animate-pulse"
          >
            Reading Immutable Chain...
          </div>
        {:else if logs.length === 0}
          <div
            class="flex items-center justify-center h-full text-neutral-light/30"
          >
            No system events found.
          </div>
        {:else}
          <div class="space-y-2">
            {#each logs as log}
              <div
                class="p-3 bento-border bg-neutral-dark/20 rounded-lg group hover:bg-neutral-dark/40 transition-colors"
              >
                <div class="flex items-center justify-between mb-1">
                  <div class="flex items-center gap-2">
                    <span class="text-primary font-bold">[{log.action}]</span>
                    <span class="text-neutral-light/40">{log.timestamp}</span>
                  </div>
                  <div class="flex items-center gap-3">
                    <span class="text-neutral-light/60">UID: {log.user}</span>
                    <span
                      class="px-1.5 py-0.5 bg-primary/10 text-primary border border-primary/20 rounded text-[8px]"
                      >{log.agency_id}</span
                    >
                  </div>
                </div>
                <p class="text-neutral-light/80 leading-relaxed mb-1 italic">
                  "{log.metadata}"
                </p>
                <div
                  class="flex items-center gap-2 text-[8px] text-neutral-light/30"
                >
                  <span class="uppercase">Hash:</span>
                  <span class="truncate max-w-[200px]">{log.curr_hash}</span>
                  <span class="ml-auto text-green-500/50">Chain Verified</span>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Footer State -->
      <div
        class="p-3 border-t border-primary/10 bg-background-dark text-[9px] text-neutral-light/40 flex justify-between italic"
      >
        <span>SHA-256 Chained Ledger</span>
        <span>Total Entries: {logs.length}</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .bento-border {
    border: 1px solid rgba(212, 178, 53, 0.1);
  }
</style>

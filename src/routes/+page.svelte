<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import BentoGrid from "$lib/components/layout/BentoGrid.svelte";
  import SystemMonitor from "$lib/components/modules/SystemMonitor.svelte";
  import FileExplorer from "$lib/components/modules/FileExplorer.svelte";
  import XtermShell from "$lib/components/terminal/XtermShell.svelte";
  import { bridge } from "$lib/bridge";
  import { agencyStore } from "$lib/stores/agency";
  import { Power, Building, ShieldCheck } from "lucide-svelte";
  import AuditLogViewer from "$lib/components/modules/AuditLogViewer.svelte";

  import { listen } from "@tauri-apps/api/event";

  /**
   * REACTIVE STATE (Runes)
   * These variables trigger UI updates when modified by either
   * event listeners or proactive bridge polling.
   */
  let kernelReady = $state(false); // Controls the splash screen visibility
  let ecoMode = $state(false); // Throttles expensive CSS effects when CPU > 80%
  let isLocked = $state(false); // Blocks bridge communication when active
  let securityStatus = $state("GREEN"); // System integrity indicator
  let status = $state({ pulse: "FAIL", latency: 0 }); // Kernel heartbeat status
  let showAudit = $state(false); // Controls the visibility of the audit log viewer
  let cpuUsage = $state(0); // Raw CPU telemetry data

  onMount(() => {
    agencyStore.init();
    bridge.startHeartbeat((newStatus) => {
      status = newStatus;
    });

    // Kernel Readiness
    listen("kora-kernel-ready", () => {
      kernelReady = true;
    });

    // Proactive Check (Race condition protection)
    bridge.koraKernelStatus().then((ready) => {
      if (ready) kernelReady = true;
    });

    // Eco-Mode Telemetry
    listen("system-telemetry", (event: any) => {
      cpuUsage = event.payload.cpu_usage;
      ecoMode = cpuUsage > 80;
    });

    // Integrity Check Polling
    const integrityInterval = setInterval(async () => {
      try {
        securityStatus = (await bridge.koraSecurityStatus()) as string;
        if (securityStatus === "RED") {
          isLocked = true; // Auto-lock on breach
          await bridge.setLock(true);
        }
      } catch (e) {
        console.error("Integrity check failed:", e);
      }
    }, 5000);

    return () => clearInterval(integrityInterval);
  });

  onDestroy(() => {
    bridge.stopHeartbeat();
  });

  // Toggle lock for testing/simulation
  async function toggleLock() {
    isLocked = !isLocked;
    await bridge.setLock(isLocked);
  }
</script>

<div
  class="h-screen w-screen overflow-hidden bg-background-dark text-neutral-light font-display relative select-none flex flex-col"
>
  <!-- Scanline Overlay (Disabled in Eco-Mode) -->
  {#if !ecoMode}
    <div class="scanline"></div>
  {/if}

  <!-- Splash Screen -->
  {#if !kernelReady}
    <div
      class="fixed inset-0 z-[1000] bg-background-dark flex flex-col items-center justify-center"
    >
      <div class="flex flex-col items-center gap-4 animate-pulse">
        <div
          class="size-16 border-2 border-primary/20 rounded-full flex items-center justify-center relative overflow-hidden"
        >
          <div
            class="absolute inset-0 bg-primary/10 animate-[ping_2s_infinite]"
          ></div>
          <span class="text-primary font-bold text-xl font-mono">K</span>
        </div>
        <div class="flex flex-col items-center gap-1">
          <span
            class="text-[10px] font-mono text-primary uppercase tracking-[0.3em]"
            >Ignition Sequence</span
          >
          <div class="w-32 h-1 bg-primary/10 rounded-full overflow-hidden">
            <div class="h-full bg-primary animate-[shimmer_2s_infinite]"></div>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Background Grid -->
  <div class="fixed inset-0 pointer-events-none opacity-[0.03] z-0">
    <div
      class="w-full h-full bg-[radial-gradient(#d4b235_1px,transparent_1px)] [background-size:20px_20px]"
    ></div>
  </div>

  <!-- Header -->
  <header
    class="flex items-center justify-between h-10 px-6 border-b border-primary/20 bg-background-dark/80 backdrop-blur-sm z-20 w-full shrink-0"
  >
    <nav class="flex items-center gap-6">
      <span
        class="text-xs font-mono uppercase tracking-widest text-neutral-light/60 hover:text-primary transition-colors cursor-pointer"
        >File</span
      >
      <span
        class="text-xs font-mono uppercase tracking-widest text-neutral-light/60 hover:text-primary transition-colors cursor-pointer"
        >Tools</span
      >
      <span
        class="text-xs font-mono uppercase tracking-widest text-neutral-light/60 hover:text-primary transition-colors cursor-pointer"
        >Knowledge</span
      >
      <span
        class="text-xs font-mono uppercase tracking-widest text-neutral-light/60 hover:text-primary transition-colors cursor-pointer"
        >System</span
      >
    </nav>
    <div class="flex items-center gap-4">
      <!-- Agency Selector -->
      <div
        class="flex items-center gap-2 mr-4 border-r border-primary/20 pr-4 h-5"
      >
        <Building size={12} class="text-[#D4B235]" />
        <select
          class="bg-transparent text-[#D4B235] text-[10px] font-mono uppercase tracking-widest border-none focus:ring-0 cursor-pointer outline-none appearance-none"
          value={$agencyStore.activeAgencyId}
          onchange={(e) => agencyStore.switchContext(e.currentTarget.value)}
        >
          {#each $agencyStore.agencies as agency}
            <option
              value={agency.id}
              class="bg-background-dark text-neutral-light"
              >{agency.name}</option
            >
          {/each}
          {#if $agencyStore.agencies.length === 0}
            <option value="SYSTEM" class="bg-background-dark text-neutral-light"
              >SYSTEM</option
            >
          {/if}
        </select>
      </div>

      <!-- Safe Exit -->
      <button
        onclick={() => agencyStore.safeExit()}
        class="hover:drop-shadow-[0_0_8px_rgba(194,59,34,0.5)] transition-all duration-300 text-neutral-400 hover:text-[#C23B22] mr-2"
        title="Safe Exit"
      >
        <Power size={14} />
      </button>

      <span class="text-xs font-mono font-bold text-primary tracking-tighter"
        >KORA OS v1.0</span
      >
      <div
        class={`size-2 rounded-full ${status.pulse === "OK" ? "bg-primary animate-pulse" : "bg-rust"}`}
      ></div>
    </div>
  </header>

  <!-- Main Content (Bento Grid) -->
  <div class="flex-1 overflow-hidden relative z-10">
    <BentoGrid>
      <div slot="sidebar" class="flex flex-col gap-4 h-full">
        <SystemMonitor />
        <!-- Agent Status Module -->
        <section class="p-4 bento-border bg-neutral-dark/40 rounded">
          <div class="flex items-center justify-between">
            <div class="flex flex-col">
              <span
                class="text-[10px] font-mono text-neutral-light/50 uppercase tracking-tighter"
                >AI Agent Status</span
              >
              <h3 class="text-sm font-mono font-bold text-neutral-light">
                OpenClaw: <span class="text-primary">Idle</span>
              </h3>
            </div>
            <div class="size-3 bg-green-500 rounded-full glow-green"></div>
          </div>
          <div
            class="mt-3 py-2 border-t border-primary/10 flex items-center gap-2"
          >
            <span class="text-primary text-xs font-mono">●</span>
            <span class="text-[10px] font-mono text-neutral-light/40"
              >Neural Engine Standby</span
            >
          </div>
        </section>
        <FileExplorer />
      </div>

      <div slot="main" class="flex-1 flex flex-col h-full relative">
        <!-- Terminal Header -->
        <div
          class="flex items-center justify-between px-4 py-2 border-b border-primary/20 bg-background-dark/60 shrink-0"
        >
          <div class="flex items-center gap-2">
            <span class="text-primary text-sm font-bold">➢</span>
            <h1 class="text-xs font-mono font-bold text-primary">
              KORA Kernel Shell
            </h1>
          </div>

          <div class="flex gap-2">
            <div class="size-2 rounded-full border border-primary/40"></div>
            <div class="size-2 rounded-full border border-primary/40"></div>
            <button
              onclick={toggleLock}
              class="text-[9px] text-rust border border-rust/30 px-1 rounded hover:bg-rust/10 uppercase ml-2 transition-colors"
            >
              {isLocked ? "UNLOCK" : "LOCK BRIDGE"}
            </button>
          </div>
        </div>

        <!-- Terminal Container -->
        <div class="flex-1 relative overflow-hidden bg-neutral-dark/20">
          <XtermShell {isLocked} />

          <AuditLogViewer
            isOpen={showAudit}
            onToggle={() => (showAudit = false)}
          />

          {#if securityStatus === "RED"}
            <div
              class="absolute inset-0 border-2 border-rust animate-pulse pointer-events-none z-[60]"
            ></div>
          {/if}

          {#if isLocked}
            <div
              class="absolute inset-0 bg-background-dark/60 backdrop-blur-sm flex items-center justify-center z-50"
            >
              <div
                class="bg-neutral-dark border border-rust/50 p-6 rounded-lg shadow-2xl text-center relative overflow-hidden"
              >
                <div
                  class="absolute top-0 left-0 w-full h-1 bg-rust animate-pulse"
                ></div>
                <h2
                  class="text-rust font-bold text-xl tracking-[0.2em] mb-2 font-mono"
                >
                  SYSTEM FREEZE
                </h2>
                <p class="text-neutral-light/60 text-xs font-mono mb-4">
                  RING 0 AUTHORIZATION REQUIRED
                </p>
                <div class="text-[10px] text-rust/60 font-mono">
                  ERROR_CODE: KERNEL_LOCK_0x99
                </div>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </BentoGrid>
  </div>

  <!-- Footer -->
  <footer
    class="h-8 px-6 bg-background-dark border-t border-primary/20 flex items-center justify-between text-[10px] font-mono uppercase tracking-widest overflow-hidden shrink-0 z-20"
  >
    <div class="flex items-center gap-4">
      <div class="flex items-center gap-2 text-neutral-light/50">
        <span>Terminal</span>
        <span>Path: ~/projects/kora</span>
      </div>
      <div class="w-px h-3 bg-primary/20"></div>
      <div class="flex items-center gap-2 text-neutral-light/50">
        <span>Git: <span class="text-primary">main*</span></span>
      </div>
    </div>
    <div class="flex items-center gap-6">
      {#if ecoMode}
        <div
          class="flex items-center gap-2 px-3 py-1 bg-orange-500/10 text-orange-500 border-l border-primary/20"
        >
          <span class="animate-pulse">ECO-MODE ACTIVE</span>
        </div>
      {/if}

      <!-- Security Dashboard Widget -->
      <button
        onclick={() => (showAudit = !showAudit)}
        class="flex items-center gap-2 px-3 py-1 border-l border-r border-primary/20 transition-all duration-300 hover:bg-white/5 {securityStatus ===
        'GREEN'
          ? 'text-emerald-500'
          : securityStatus === 'ORANGE'
            ? 'text-orange-500'
            : 'text-rust font-black animate-bounce'}"
        title="View Audit Logs"
      >
        <ShieldCheck size={12} />
        <div
          class="size-1.5 rounded-full {securityStatus === 'GREEN'
            ? 'bg-emerald-500 shadow-[0_0_8px_#10b981]'
            : securityStatus === 'ORANGE'
              ? 'bg-orange-500'
              : 'bg-rust shadow-[0_0_12px_#ef4444]'} {securityStatus !== 'GREEN'
            ? 'animate-pulse'
            : ''}"
        ></div>
        <span class="whitespace-nowrap"
          >{securityStatus === "GREEN"
            ? "KER_SEC_OK"
            : securityStatus === "ORANGE"
              ? "SEC_WARN"
              : "INTEGRITY_BREACH"}</span
        >
      </button>

      <div
        class="flex items-center gap-2 px-3 py-1 bg-primary/10 text-primary border-r border-primary/20 animate-pulse"
      >
        <div class="size-1.5 bg-primary rounded-full"></div>
        <span class="font-bold">Ready</span>
      </div>
    </div>
  </footer>
</div>

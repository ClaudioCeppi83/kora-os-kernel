<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { Cpu, MemoryStick } from "lucide-svelte";

  interface SystemMetrics {
    cpu_usage: number;
    ram_used: number;
    ram_total: number;
    timestamp: string;
  }

  let cpuUsage = 0;
  let ramUsed = 0;
  let ramTotal = 32 * 1024 * 1024 * 1024; // Default to 32GB until update

  // 30 data points for the graph
  let cpuHistory: number[] = new Array(30).fill(0);
  let ramHistory: number[] = new Array(30).fill(0);

  let unlisten: () => void;

  onMount(async () => {
    unlisten = await listen<SystemMetrics>("system-telemetry", (event) => {
      const metrics = event.payload;
      cpuUsage = metrics.cpu_usage;
      ramUsed = metrics.ram_used;
      ramTotal = metrics.ram_total;

      cpuHistory = [...cpuHistory.slice(1), cpuUsage];

      const ramPercent = (ramUsed / ramTotal) * 100;
      ramHistory = [...ramHistory.slice(1), ramPercent];
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });

  $: cpuPath =
    `M0 64 ` +
    cpuHistory
      .map((val, i) => `L${i * 10} ${64 - (val / 100) * 64}`)
      .join(" ") +
    ` L300 64 Z`;
  $: ramPath =
    `M0 64 ` +
    ramHistory
      .map((val, i) => `L${i * 10} ${64 - (val / 100) * 64}`)
      .join(" ") +
    ` L300 64 Z`;

  $: ramAppStr = (ramUsed / (1024 * 1024 * 1024)).toFixed(1);
  $: ramTotalStr = (ramTotal / (1024 * 1024 * 1024)).toFixed(0);
</script>

<section
  class="flex flex-col gap-4 p-4 bento-border bg-neutral-dark/40 rounded shadow-sm"
>
  <div class="flex items-center gap-2 mb-2">
    <span class="text-primary"><Cpu size={16} /></span>
    <h2
      class="text-xs font-mono font-bold uppercase tracking-widest text-neutral-light/80"
    >
      System Monitor
    </h2>
  </div>

  <!-- CPU Graph -->
  <div class="flex flex-col gap-1">
    <div class="flex justify-between items-end">
      <span class="text-[10px] font-mono text-neutral-light/50"
        >CPU UTILIZATION</span
      >
      <span class="text-xs font-mono text-primary">{cpuUsage.toFixed(1)}%</span>
    </div>
    <div
      class="h-16 w-full relative overflow-hidden bg-background-dark/50 border border-primary/10"
    >
      <svg
        class="absolute inset-0 w-full h-full"
        preserveAspectRatio="none"
        viewBox="0 0 300 64"
      >
        <path
          d={cpuPath}
          fill="rgba(212, 178, 53, 0.1)"
          stroke="#d4b235"
          stroke-width="1"
        ></path>
      </svg>
    </div>
  </div>

  <!-- RAM Graph -->
  <div class="flex flex-col gap-1">
    <div class="flex justify-between items-end">
      <span class="text-[10px] font-mono text-neutral-light/50">RAM USAGE</span>
      <span class="text-xs font-mono text-primary"
        >{ramAppStr} / {ramTotalStr}GB</span
      >
    </div>
    <div
      class="h-16 w-full relative overflow-hidden bg-background-dark/50 border border-primary/10"
    >
      <svg
        class="absolute inset-0 w-full h-full"
        preserveAspectRatio="none"
        viewBox="0 0 300 64"
      >
        <path
          d={ramPath}
          fill="rgba(194, 59, 34, 0.1)"
          stroke="#C23B22"
          stroke-width="1"
        ></path>
      </svg>
    </div>
  </div>
</section>

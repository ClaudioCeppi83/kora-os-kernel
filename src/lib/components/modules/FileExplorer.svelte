<script lang="ts">
  import { Folder, FileText } from "lucide-svelte";

  // Mock data structure matching code.html for now
  const fileSystem = [
    {
      name: "knowledge",
      type: "folder",
      color: "text-primary",
      children: [
        { name: "neural_schema.md", type: "file" },
        { name: "system_core.json", type: "file" },
      ],
    },
    {
      name: "workspace",
      type: "folder",
      color: "text-neutral-light",
      children: [],
    },
    {
      name: "logs",
      type: "folder",
      color: "text-rust",
      children: [],
    },
  ];
</script>

<section
  class="flex-1 p-4 bento-border bg-neutral-dark/40 rounded overflow-hidden flex flex-col"
>
  <div class="flex items-center gap-2 mb-4">
    <span class="text-primary"><Folder size={16} /></span>
    <h2
      class="text-xs font-mono font-bold uppercase tracking-widest text-neutral-light/80"
    >
      File Explorer
    </h2>
  </div>

  <div class="flex flex-col gap-3 font-mono text-xs">
    {#each fileSystem as item}
      <div>
        <div
          class={`flex items-center gap-2 cursor-pointer hover:bg-neutral-light/10 py-1 rounded px-1 ${item.color}`}
        >
          <Folder size={14} />
          <span>{item.name}</span>
        </div>
        {#if item.children && item.children.length > 0}
          <div
            class="flex flex-col gap-2 ml-4 border-l border-primary/20 pl-4 mt-1"
          >
            {#each item.children as child}
              <div
                class="flex items-center gap-2 text-neutral-light/60 hover:text-neutral-light cursor-pointer"
              >
                <FileText size={14} />
                <span>{child.name}</span>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </div>
</section>

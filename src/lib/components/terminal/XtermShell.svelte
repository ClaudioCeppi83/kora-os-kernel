<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import { WebglAddon } from "@xterm/addon-webgl";
  import "@xterm/xterm/css/xterm.css";
  import { bridge } from "$lib/bridge";

  let termContainer: HTMLElement;
  let term: Terminal;
  let fitAddon: FitAddon;
  let unlisten: () => void;
  let resizeObserver: ResizeObserver;

  export let isLocked = false;

  $: if (term) {
    if (isLocked) {
      term.options.theme = { ...term.options.theme, cursor: "#C23B22" };
      term.options.cursorStyle = "block";
      term.blur();
    } else {
      term.options.theme = { ...term.options.theme, cursor: "#D4B235" };
      term.options.cursorStyle = "block";
      term.focus();
    }
  }

  onMount(async () => {
    term = new Terminal({
      fontFamily: '"JetBrains Mono", monospace',
      fontSize: 13,
      letterSpacing: 0,
      lineHeight: 1.2,
      cursorBlink: true,
      allowTransparency: true,
      theme: {
        background: "#00000000",
        foreground: "#E6E6E6",
        cursor: "#D4B235",
        selectionBackground: "rgba(212, 178, 53, 0.3)",
        black: "#121212",
        red: "#C23B22",
        green: "#23D18B",
        yellow: "#D4B235",
        blue: "#2472C8",
        magenta: "#BC3FBC",
        cyan: "#59C2C6",
        white: "#E6E6E6",
        brightBlack: "#666666",
        brightRed: "#F14C4C",
        brightGreen: "#23D18B",
        brightYellow: "#F5F543",
        brightBlue: "#3B8EEA",
        brightMagenta: "#D670D6",
        brightCyan: "#29B8DB",
        brightWhite: "#E5E5E5",
      },
    });

    fitAddon = new FitAddon();
    term.loadAddon(fitAddon);

    // WebGL Addon
    const webgl = new WebglAddon();
    webgl.onContextLoss((e) => {
      webgl.dispose();
    });
    term.loadAddon(webgl);

    term.open(termContainer);
    fitAddon.fit();

    // Data handling
    try {
      unlisten = await bridge.listenPty((data) => {
        term.write(data);
      });
      // Listen for AI Engine Output
      const unlistenOutput = await bridge.listen(
        "openclaw-output",
        (event: any) => {
          term.write(`\r\n\x1b[38;2;212;178;53m[AI] ${event.payload}\x1b[0m`);
          // term.write("\r\n$ "); // Restore prompt if needed, but risky
        },
      );
      const unlistenError = await bridge.listen(
        "openclaw-error",
        (event: any) => {
          term.write(`\r\n\x1b[31m[AI ERR] ${event.payload}\x1b[0m`);
        },
      );
    } catch (e) {
      console.error("Failed to listen to PTY/AI:", e);
      term.write(
        "\r\n\x1b[31m[ERROR] Native Bridge Connection Failed.\x1b[0m\r\n",
      );
    }

    let currentLine = "";

    term.onData((data) => {
      if (isLocked) return;

      // Simple line buffering for command interception
      if (data === "\r") {
        const trimmed = currentLine.trim();
        if (trimmed.startsWith("kora ")) {
          // Intercept KORA commands
          term.write("\r\n"); // Echo newline locally

          const parts = trimmed.split(" ");
          const cmd = parts[1]; // system or knowledge
          const args = parts.slice(2).join(" ");

          if (cmd === "system") {
            if (args === "benchmark") {
              term.write(
                "\x1b[38;2;212;178;53m[Initiating High-Performance Benchmark...]\x1b[0m\r\n",
              );
              bridge
                .koraSystemBenchmark()
                .then((res) => {
                  term.write(
                    "\r\n\x1b[1;38;2;212;178;53mKORA OS PERFORMANCE REPORT\x1b[0m\r\n",
                  );
                  term.write(
                    "\x1b[38;2;212;178;53m--------------------------------\x1b[0m\r\n",
                  );
                  term.write(
                    `\x1b[33mBoot Latency:\x1b[0m   ${res.boot_ms}ms\r\n`,
                  );
                  term.write(
                    `\x1b[33mActive RAM:\x1b[0m     ${res.ram_mb}MB\r\n`,
                  );
                  term.write(
                    `\x1b[33mDB Latency:\x1b[0m     ${res.db_latency_us}µs\r\n`,
                  );
                  term.write(
                    `\x1b[33mZero-Copy RAG:\x1b[0m  ${res.zero_copy_rag ? "ENABLED" : "DISABLED"}\r\n`,
                  );
                  term.write(
                    `\x1b[33mLTO/Hardening:\x1b[0m  ${res.lto ? "ACTIVE" : "INACTIVE"}\r\n`,
                  );
                  term.write(
                    "\x1b[38;2;212;178;53m--------------------------------\x1b[0m\r\n",
                  );
                  term.write(`\r\n$ `);
                })
                .catch((e) => {
                  term.write(
                    `\r\n\x1b[31m✖ Benchmark Failed: ${e}\x1b[0m\r\n$ `,
                  );
                });
            } else {
              term.write(
                "\x1b[38;2;100;100;100m[Processing System Command...]\x1b[0m\r\n",
              );
              bridge
                .koraSystem(args)
                .then((res) => {
                  term.write(`\r\n$ `);
                })
                .catch((e) => {
                  term.write(`\r\n\x1b[31m✖ Error: ${e}\x1b[0m\r\n$ `);
                });
            }
          } else if (cmd === "knowledge") {
            term.write(
              "\x1b[38;2;100;100;100m[Querying Neural Link...]\x1b[0m\r\n",
            );
            bridge
              .koraKnowledge(args)
              .then((res) => {
                // Response comes via event, just restore prompt or wait
                // term.write(`\r\n$ `);
              })
              .catch((e) => {
                term.write(`\r\n\x1b[31m✖ Error: ${e}\x1b[0m\r\n$ `);
              });
          } else if (cmd === "agency") {
            const subCmd = args.split(" ")[0];
            const subArgs = args.substring(subCmd.length).trim();

            if (subCmd === "create") {
              term.write(
                "\x1b[38;2;100;100;100m[Initiating Agency Protocol...]\x1b[0m\r\n",
              );
              bridge
                .koraAgencyCreate(subArgs)
                .then((res) => term.write(`\r\n\x1b[32m✔ ${res}\x1b[0m\r\n$ `))
                .catch((e) =>
                  term.write(`\r\n\x1b[31m✖ Error: ${e}\x1b[0m\r\n$ `),
                );
            } else if (subCmd === "list") {
              bridge
                .koraAgencyList()
                .then((list: any[]) => {
                  term.write("\r\n");
                  list.forEach((a) => term.write(`- ${a.id} (${a.name})\r\n`));
                  term.write("$ ");
                })
                .catch((e) =>
                  term.write(`\r\n\x1b[31m✖ Error: ${e}\x1b[0m\r\n$ `),
                );
            } else if (subCmd === "switch") {
              term.write(
                "\x1b[38;2;212;178;53m[Switching Context...]\x1b[0m\r\n",
              );
              bridge
                .koraAgencySwitch(subArgs)
                .then((res) => term.write(`\r\n\x1b[32m✔ ${res}\x1b[0m\r\n$ `))
                .catch((e) =>
                  term.write(`\r\n\x1b[31m✖ Error: ${e}\x1b[0m\r\n$ `),
                );
            } else {
              term.write(
                `\r\n\x1b[31mUnknown agency command: ${subCmd}\x1b[0m\r\n$ `,
              );
            }
          } else {
            term.write(`\r\n\x1b[31mUnknown kora command: ${cmd}\x1b[0m\r\n$ `);
          }

          currentLine = "";
          return; // STOP here, do not send \r to PTY
        }
        currentLine = ""; // Reset on other commands
      } else if (data === "\u007F") {
        // Backspace
        if (currentLine.length > 0) {
          currentLine = currentLine.slice(0, -1);
        }
      } else if (data >= " ") {
        // Printable
        currentLine += data;
      }

      // Pass through to PTY (unless intercepted above)
      bridge.ptyWrite(data).catch((e) => {
        console.error("PTY Write Error:", e);
      });
    });

    // Resize handling
    resizeObserver = new ResizeObserver(() => {
      fitAddon.fit();
    });
    resizeObserver.observe(termContainer);

    // Focus terminal on click
    termContainer.addEventListener("click", () => {
      if (!isLocked) term.focus();
    });

    // Initial focus
    term.focus();
    term.write(
      "\r\n\x1b[38;2;212;178;53mKORA Shell activo (Ring 1). Comandos de sistema restringidos a esta interfaz.\x1b[0m\r\n$ ",
    );
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    // We should unlisten the others too, but unlisten variable is single-function currently.
    // Ideally we track all unlisteners.
    if (resizeObserver) resizeObserver.disconnect();
    if (term) term.dispose();
  });
</script>

<div class="h-full w-full p-1" bind:this={termContainer}></div>

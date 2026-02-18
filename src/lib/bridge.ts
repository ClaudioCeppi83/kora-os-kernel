import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface BridgeStatus {
  pulse: "OK" | "FAIL";
  latency: number;
}

class KoraBridge {
  private heartbeatInterval: number | null = null;
  public status: BridgeStatus = { pulse: "FAIL", latency: 0 };

  async ptyWrite(data: string) {
    await invoke("pty_write", { data });
  }

  async setLock(locked: boolean) {
    await invoke("set_bridge_lock", { locked });
  }

  async listenPty(callback: (data: string) => void) {
    return await listen<string>("pty-data", (event) => {
      callback(event.payload);
    });
  }

  async sendNotification(title: string, body: string) {
      await invoke("send_notification", { title, body });
  }

  async indexFile(path: string): Promise<string> {
    return await invoke("index_file", { path });
  }

  async getAuditLogs(): Promise<any[]> {
    return await invoke("get_audit_logs");
  }

  startHeartbeat(onStatusChange: (status: BridgeStatus) => void) {
    if (this.heartbeatInterval) return;

    this.heartbeatInterval = window.setInterval(async () => {
      const start = Date.now();
      try {
        const res = await invoke<string>("heartbeat");
        if (res === "PULSE_OK") {
          this.status = {
            pulse: "OK",
            latency: Date.now() - start
          };
        }
      } catch (e) {
        this.status = { pulse: "FAIL", latency: -1 };
      }
      onStatusChange(this.status);
    }, 100);
  }

  // Generic listener
  async listen(event: string, callback: (payload: any) => void) {
    return await listen<any>(event, (e) => callback(e));
  }

  async koraKernelStatus(): Promise<boolean> {
    try {
      return await invoke<boolean>("kora_kernel_status");
    } catch (e) {
      return false;
    }
  }

  // Phase 6 Commands
  async koraSystem(action: string): Promise<string> {
    return await invoke("kora_system", { action });
  }

  async koraSystemBenchmark(): Promise<any> {
    return await invoke("kora_system_benchmark");
  }

  async koraKnowledge(query: string): Promise<string> {
    return await invoke("kora_knowledge", { query });
  }

  // Phase 7: Governance & Multi-Tenancy
  async koraAgencyCreate(name: string): Promise<string> {
    return await invoke("kora_agency_create", { name });
  }

  async koraAgencyList(): Promise<any[]> {
    return await invoke("kora_agency_list");
  }

  async koraAgencySwitch(id: string): Promise<string> {
    return await invoke("kora_agency_switch", { id });
  }

  async koraSecurityStatus(): Promise<string> {
    return await invoke("kora_kernel_integrity");
  }

  async koraSafeExit(): Promise<void> {
    return await invoke("cmd_shutdown");
  }

  stopHeartbeat() {
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
  }
}

export const bridge = new KoraBridge();

import { writable } from 'svelte/store';
import { bridge } from '../bridge';

export interface Agency {
  id: string;
  name: string;
  created_at: string;
}

interface AgencyState {
  activeAgencyId: string;
  agencies: Agency[];
  isLoading: boolean;
}

function createAgencyStore() {
  const { subscribe, set, update } = writable<AgencyState>({
    activeAgencyId: 'SYSTEM',
    agencies: [],
    isLoading: false
  });

  return {
    subscribe,
    
    init: async () => {
      update(s => ({ ...s, isLoading: true }));
      try {
        const list: Agency[] = await bridge.koraAgencyList();
        set({
            activeAgencyId: 'SYSTEM', 
            agencies: list, 
            isLoading: false 
        });

        // Listen for backend context switches (e.g. via CLI)
        await bridge.listen("context-switching", (event: any) => {
             // event.payload should be the new ID string
             const newId = typeof event.payload === 'string' ? event.payload : event.payload.message; 
             update(s => ({ ...s, activeAgencyId: newId }));
        });

      } catch (e) {
        console.error("Agency Init Failed", e);
        update(s => ({ ...s, isLoading: false }));
      }
    },

    create: async (name: string) => {
      update(s => ({ ...s, isLoading: true }));
      try {
        await bridge.koraAgencyCreate(name);
        const list: Agency[] = await bridge.koraAgencyList();
        
        update(s => ({ 
            ...s, 
            agencies: list, 
            isLoading: false 
        }));
        return true;
      } catch (e) {
        console.error("Agency Create Failed", e);
        update(s => ({ ...s, isLoading: false }));
        return false;
      }
    },
    
    switchContext: async (id: string) => {
      update(s => ({ ...s, isLoading: true }));
      try {
        await bridge.koraAgencySwitch(id);
        update(s => ({ ...s, activeAgencyId: id, isLoading: false }));
      } catch (e) {
        console.error("Agency Switch Failed", e);
        update(s => ({ ...s, isLoading: false }));
      }
    },

    safeExit: async () => {
        await bridge.koraSafeExit();
    }
  };
}

export const agencyStore = createAgencyStore();

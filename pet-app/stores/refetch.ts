import { defineStore } from 'pinia';

export const useRefetchStore = defineStore('refetch', {
    state: () => ({
        needRefetch: false
    }),
    actions: {
        triggerRefetch() {
            this.needRefetch = !this.needRefetch;
        }
    }
});

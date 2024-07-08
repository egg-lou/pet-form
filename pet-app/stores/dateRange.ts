import {defineStore} from "pinia";

export const useDateRangeStore =defineStore('dateRange', {
    state: () => ({
        start: '',
        end: ''
    }),
    actions: {
        updateDateRange(start, end) {
            this.start = start;
            this.end = end;
        }
    }
})
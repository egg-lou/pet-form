import { defineStore } from 'pinia';

export const useFormCheckStore = defineStore('form', {
    state: () => ({
        triggerValidation: false,
        isValid: false
    }),
    actions: {
        validateForm() {
            this.triggerValidation = !this.triggerValidation;
        },
        setValid() {
            this.isValid = !this.isValid;
        },
        setInvalid() {
            this.isValid = false;
        }
    }
});

import { inject, ref } from 'vue'
import { defineStore } from 'pinia'

export const useStateStore = defineStore('state', {
    state: () => {
        return {
            theme: 'dark',
        }
    }
})
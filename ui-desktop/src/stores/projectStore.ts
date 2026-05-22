import { defineStore } from 'pinia'

export const useProjectStore = defineStore('project', {
  state: () => ({
    currentModel: null,
    unfoldResult: null
  }),
  actions: {
    setModel(model) { this.currentModel = model },
    setUnfoldResult(result) { this.unfoldResult = result }
  }
})

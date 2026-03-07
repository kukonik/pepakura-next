import { ref } from 'vue'
import type { NotificationToast } from '../components/NotificationToast.vue'

export function useNotifications() {
  const toastRef = ref<InstanceType<typeof NotificationToast> | null>(null)

  const notify = (message: string, type: 'success' | 'error' | 'info' = 'info') => {
    if (toastRef.value) {
      toastRef.value.addToast(message, type)
    } else {
      console[type === 'error' ? 'error' : 'log'](message)
    }
  }

  const notifySuccess = (message: string) => notify(message, 'success')
  const notifyError = (message: string) => notify(message, 'error')
  const notifyInfo = (message: string) => notify(message, 'info')

  return {
    toastRef,
    notify,
    notifySuccess,
    notifyError,
    notifyInfo
  }
}

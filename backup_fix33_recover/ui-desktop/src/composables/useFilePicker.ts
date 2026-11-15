import { ref } from 'vue'

export const useFilePicker = () => {
  const fileInput = ref<HTMLInputElement | null>(null)
  
  // Универсальный метод открытия файлов
  const openFile = async (options: { 
    multiple?: boolean; 
    accept?: string; 
    title?: string 
  } = {}) => {
    return new Promise<string[] | null>((resolve) => {
      // Попытка использовать Tauri API (в нативном окне)
      try {
        import('@tauri-apps/api/dialog').then(async ({ open }) => {
          const selected = await open({
            multiple: options.multiple ?? false,
            filters: options.accept 
              ? [{ name: 'Files', extensions: options.accept.split(',').map(ext => ext.trim().replace(/^\./, '')) }]
              : undefined
          })
          
          if (selected) {
            resolve(Array.isArray(selected) ? selected : [selected])
          } else {
            resolve(null)
          }
        }).catch(() => {
          // Если Tauri недоступен — использовать веб-API
          fallbackToFileInput(resolve, options)
        })
      } catch {
        // Если импорт не удался — использовать веб-API
        fallbackToFileInput(resolve, options)
      }
    })
  }
  
  // Резервный метод через <input type="file">
  const fallbackToFileInput = (
    resolve: (files: string[] | null) => void,
    options: { multiple?: boolean; accept?: string; title?: string }
  ) => {
    if (!fileInput.value) {
      fileInput.value = document.createElement('input')
      fileInput.value.type = 'file'
      fileInput.value.style.display = 'none'
      document.body.appendChild(fileInput.value)
      
      fileInput.value.addEventListener('change', (e) => {
        const files = (e.target as HTMLInputElement).files
        if (files && files.length > 0) {
          const paths = Array.from(files).map(f => URL.createObjectURL(f))
          resolve(paths)
        } else {
          resolve(null)
        }
        // Очистить значение для повторного выбора
        fileInput.value!.value = ''
      })
    }
    
    fileInput.value.multiple = options.multiple ?? false
    if (options.accept) fileInput.value.accept = options.accept
    fileInput.value.click()
  }
  
  return { openFile }
}

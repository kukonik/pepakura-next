import { ref } from 'vue'
import { useProjectStore } from '@/stores/project'

type MessageLevel = 'info' | 'error' | 'success'

export interface UiMessage {
  id: string
  text: string
  level: MessageLevel
}

const messages = ref<UiMessage[]>([])

function pushMessage(text: string, level: MessageLevel = 'info') {
  const msg: UiMessage = {
    id: crypto.randomUUID(),
    text,
    level
  }
  messages.value.push(msg)
  setTimeout(() => {
    messages.value = messages.value.filter(m => m.id !== msg.id)
  }, 4000)
}

export function useTauriModelLoader() {
  const project = useProjectStore()

  async function open3DModelDialog(): Promise<null> {
    pushMessage(
      'Диалог выбора файла Tauri пока не подключён. Используйте drag&drop или кнопку выбора файлов в зоне 3D.',
      'info'
    )
    return null
  }

  async function handle3DFilesDrop(files: FileList | File[]): Promise<void> {
    if (!files || files.length === 0) {
      pushMessage('Файлы не выбраны', 'error')
      return
    }

    const list = Array.from(files)
    const objFile = list.find(f => f.name.toLowerCase().endsWith('.obj'))
    const mtlFile = list.find(f => f.name.toLowerCase().endsWith('.mtl'))

    if (!objFile) {
      pushMessage('Не найден OBJ‑файл среди переданных файлов', 'error')
      project.markThreeDError('OBJ‑файл не найден')
      return
    }

    // Заглушка: считаем, что workingPath = имя OBJ
    const sourcePath = objFile.name
    const workingPath = objFile.name

    project.setThreeD({
      sourcePath,
      workingPath,
      format: 'obj+mtl',
      status: 'ready',
      lastError: null
    })

    // Заглушечные значения, чтобы UI был живым
    project.markThreeDReady({
      faces: 4460,
      parts: 3
    })

    pushMessage(
      `Загружен OBJ${mtlFile ? ' + MTL' : ''}: ${objFile.name}. Данные анализированы в режиме заглушки.`,
      'success'
    )
  }

  function showMessage(text: string, level: MessageLevel = 'info') {
    pushMessage(text, level)
  }

  return {
    messages,
    open3DModelDialog,
    handle3DFilesDrop,
    showMessage
  }
}

<template>
  <div style="max-width:900px;margin:2em auto;">
    <h1>Просмотр и редактор файлов</h1>
    <input type="file" multiple @change="onFileChange" accept=".svg,.obj,.png,.jpg,.jpeg,.bmp,.webp,.gif" style="margin-bottom:1em;">
    <div v-if="fileList.length > 1" style="margin-bottom:1em;">
      <strong>Коллекция:</strong>
      <button @click="prevFile" :disabled="currentIdx === 0">⟨</button>
      <span>{{ currentIdx+1 }} / {{ fileList.length }}</span>
      <button @click="nextFile" :disabled="currentIdx === fileList.length-1">⟩</button>
    </div>
    <div v-if="isSVG">
      <h2>SVG предпросмотр</h2>
      <div style="border:1px solid #ccc;padding:1em;margin-bottom:1em;min-height:200px;">
        <div v-html="svgContent"></div>
      </div>
      <textarea v-model="svgContent" rows="12" style="width:100%;font-family:monospace;"></textarea>
      <div>
        <button @click="saveSvg" style="margin-top:1em;margin-right:1em;">Сохранить как .svg</button>
        <button @click="exportSvg" style="margin-top:1em;margin-right:1em;">Экспортировать в файл</button>
        <button @click="callAi" style="margin-top:1em;margin-right:1em;">AI анализ SVG</button>
        <button @click="clearAll" style="margin-top:1em;">Очистить</button>
      </div>
    </div>
    <div v-else-if="isImage">
      <h2>2D изображение</h2>
      <div style="border:1px solid #ccc;display:inline-block;">
        <img :src="imgSrc" :style="{maxWidth:'100%',transform:`scale(${imgScale})`}" @wheel.prevent="onWheel"/>
      </div>
      <div>Масштаб: {{ imgScale }}</div>
      <button @click="resetScale" style="margin-top:1em;margin-right:1em;">Сбросить масштаб</button>
      <button @click="exportImg" style="margin-top:1em;margin-right:1em;">Экспортировать в файл</button>
      <button @click="callAi" style="margin-top:1em;margin-right:1em;">AI анализ изображения</button>
      <button @click="clearAll" style="margin-top:1em;">Очистить</button>
    </div>
    <div v-else-if="isObj">
      <h2>OBJ-файл:</h2>
      <div><strong>{{ objName }}</strong> ({{ objSize }} КБ)</div>
      <button @click="exportObj" style="margin-top:1em;margin-right:1em;">Экспортировать в файл</button>
      <button @click="callAi" style="margin-top:1em;margin-right:1em;">AI анализ OBJ</button>
      <button @click="clearAll" style="margin-top:1em;">Очистить</button>
    </div>
    <div v-if="!svgContent && !imgSrc && !objName">
      <p>Загрузите SVG, OBJ или 2D-изображение для предпросмотра и редактирования.</p>
    </div>
    <div v-if="statusMsg" style="margin-top:2em;color:green;">{{ statusMsg }}</div>
    <div v-if="aiResult" style="margin-top:2em;color:#185080;background:#f1f6ff;padding:1em;white-space:pre-wrap;">
      <strong>AI результат:</strong>
      <div>{{ aiResult }}</div>
    </div>
  </div>
</template>

<script>
export default {
  name: "ModelViewerPage",
  data() {
    return {
      svgContent: "",
      objName: "",
      objSize: 0,
      imgSrc: "",
      imgScale: 1,
      fileList: [],
      currentIdx: 0,
      statusMsg: "",
      aiResult: ""
    }
  },
  computed: {
    isSVG() { return !!this.svgContent },
    isObj() { return !!this.objName },
    isImage() { return !!this.imgSrc }
  },
  methods: {
    onFileChange(e) {
      this.fileList = Array.from(e.target.files)
      this.currentIdx = 0
      this.showFile(this.fileList[0])
    },
    showFile(file) {
      if (!file) return
      if (file.name.match(/\.(svg)$/i)) {
        const reader = new FileReader()
        reader.onload = ev => {
          this.svgContent = ev.target.result
          this.objName = ""; this.objSize = 0; this.imgSrc = ""; this.imgScale = 1
        }
        reader.readAsText(file)
      } else if (file.name.match(/\.(obj)$/i)) {
        this.objName = file.name
        this.objSize = Math.round(file.size/1024)
        this.svgContent = ""; this.imgSrc = ""; this.imgScale = 1
      } else if (file.name.match(/\.(png|jpg|jpeg|bmp|webp|gif)$/i)) {
        const reader = new FileReader()
        reader.onload = ev => {
          this.imgSrc = ev.target.result
          this.svgContent = ""; this.objName = ""; this.objSize = 0; this.imgScale = 1
        }
        reader.readAsDataURL(file)
      } else {
        alert("Допустимы: SVG, OBJ, PNG/JPG/BMP/GIF/WebP.")
      }
    },
    nextFile() {
      if (this.currentIdx < this.fileList.length - 1) {
        this.currentIdx += 1
        this.showFile(this.fileList[this.currentIdx])
      }
    },
    prevFile() {
      if (this.currentIdx > 0) {
        this.currentIdx -= 1
        this.showFile(this.fileList[this.currentIdx])
      }
    },
    clearAll() {
      this.svgContent = ""
      this.objName = ""
      this.objSize = 0
      this.imgSrc = ""
      this.imgScale = 1
      this.fileList = []
      this.currentIdx = 0
      this.statusMsg = ""
      this.aiResult = ""
    },
    onWheel(event) {
      if (event.deltaY < 0 && this.imgScale < 4) this.imgScale += 0.1
      if (event.deltaY > 0 && this.imgScale > 0.2) this.imgScale -= 0.1
      this.imgScale = Math.round(this.imgScale * 100) / 100
    },
    resetScale() {
      this.imgScale = 1
    },
    saveSvg() {
      const blob = new Blob([this.svgContent], { type: "image/svg+xml" })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = "edited.svg"
      a.click()
      URL.revokeObjectURL(url)
    },
    async exportSvg() {
      if (!window.electron || !window.electron.saveFile) {
        alert("Экспорт доступен только в Electron.");
        return;
      }
      const result = await window.electron.saveFile("edited.svg", this.svgContent, "svg")
      this.statusMsg = result.success ? "SVG успешно экспортирован." : "Ошибка экспорта SVG."
    },
    async exportObj() {
      if (!window.electron || !window.electron.saveFile) {
        alert("Экспорт доступен только в Electron.");
        return;
      }
      let filedata = ""
      for (const file of this.fileList) {
        if (file.name === this.objName) {
          filedata = await file.text()
        }
      }
      const result = await window.electron.saveFile(this.objName, filedata || "", "obj")
      this.statusMsg = result.success ? "OBJ успешно экспортирован." : "Ошибка экспорта OBJ."
    },
    async exportImg() {
      if (!window.electron || !window.electron.saveFile) {
        alert("Экспорт доступен только в Electron.");
        return;
      }
      let ext = "png"
      if (this.imgSrc.startsWith("data:image/gif")) ext = "gif"
      else if (this.imgSrc.startsWith("data:image/jpeg")) ext = "jpg"
      else if (this.imgSrc.startsWith("data:image/bmp")) ext = "bmp"
      else if (this.imgSrc.startsWith("data:image/webp")) ext = "webp"
      const base64 = this.imgSrc.split(',')[1]
      const buffer = Uint8Array.from(atob(base64), c => c.charCodeAt(0))
      const result = await window.electron.saveFile("exported." + ext, buffer, ext)
      this.statusMsg = result.success ? "Изображение успешно экспортировано." : "Ошибка экспорта изображения."
    },
    async callAi() {
      let data = {}
      if (this.isSVG) data = { type: "svg", content: this.svgContent }
      else if (this.isObj) {
        for (const file of this.fileList) {
          if (file.name === this.objName) {
            data = { type: "obj", content: await file.text() }
          }
        }
      } else if (this.isImage) data = { type: "image", dataUrl: this.imgSrc }
      const resp = await window.electron.apiRequest("post", "http://127.0.0.1:8000/api/analyze", data)
      if (resp.success) this.aiResult = JSON.stringify(resp.result, null, 2)
      else this.aiResult = "Ошибка AI запроса: " + resp.error
    }
  }
}
</script>

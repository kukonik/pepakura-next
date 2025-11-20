const { contextBridge, ipcRenderer } = require('electron');
contextBridge.exposeInMainWorld('electron', {
  saveFile: (defaultName, data, ext) =>
    ipcRenderer.invoke('save-file', { defaultName, data, ext }),
  apiRequest: (method, url, data, headers) =>
    ipcRenderer.invoke('api-request', { method, url, data, headers })
});

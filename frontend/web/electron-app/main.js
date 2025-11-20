const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const path = require('path');
const fs = require('fs');
const axios = require('axios');

function createWindow() {
  const win = new BrowserWindow({
    width: 1200,
    height: 800,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      nodeIntegration: false,
      contextIsolation: true
    }
  });
  // Исправленный путь — грузим '../vue-app/dist/index.html' относительно electron-app
  win.loadFile(path.resolve(__dirname, '../vue-app/dist/index.html'));
  win.setMenu(null);
}

app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) createWindow();
});

ipcMain.handle('save-file', async (event, { defaultName, data, ext }) => {
  const win = BrowserWindow.getFocusedWindow();
  const { filePath, canceled } = await dialog.showSaveDialog(win, {
    defaultPath: defaultName,
    filters: [{ name: ext.toUpperCase(), extensions: [ext] }]
  });
  if (canceled || !filePath) return { success: false };
  try {
    if (typeof data === "string") {
      fs.writeFileSync(filePath, data, 'utf8');
    } else {
      fs.writeFileSync(filePath, Buffer.from(data));
    }
    return { success: true };
  } catch {
    return { success: false };
  }
});

ipcMain.handle('api-request', async (event, { method, url, data, headers }) => {
  try {
    const resp = await axios({
      method: method || "post",
      url,
      data,
      headers: headers || {}
    });
    return { success: true, result: resp.data };
  } catch (err) {
    return { success: false, error: err.message, details: err.response && err.response.data };
  }
});

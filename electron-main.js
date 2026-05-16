const { app, BrowserWindow, Tray, Menu, ipcMain } = require('electron');
const path = require('path');

let tray = null;
let windows = {};

function createMainWindow() {
  const win = new BrowserWindow({
    width: 1920,
    height: 1080,
    transparent: true,
    frame: false,
    resizable: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    show: false,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    }
  });
  win.maximize();
  win.setIgnoreMouseEvents(true);
  win.loadFile('index.html');
  
  win.on('close', (e) => {
    e.preventDefault();
    win.hide();
  });
  
  return win;
}

function createMode1Window() {
  const win = new BrowserWindow({
    width: 1920,
    height: 1080,
    transparent: true,
    frame: false,
    resizable: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    show: false,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    }
  });
  win.maximize();
  win.setIgnoreMouseEvents(true);
  win.loadFile('mode1.html');
  return win;
}

function createMode2Window() {
  const win = new BrowserWindow({
    width: 1920,
    height: 1080,
    transparent: true,
    frame: false,
    resizable: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    show: false,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    }
  });
  win.maximize();
  win.setIgnoreMouseEvents(true);
  win.loadFile('mode2.html');
  return win;
}

function createMode3Window() {
  const win = new BrowserWindow({
    width: 1920,
    height: 1080,
    transparent: true,
    frame: false,
    resizable: false,
    alwaysOnTop: true,
    skipTaskbar: true,
    show: false,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    }
  });
  win.maximize();
  win.setIgnoreMouseEvents(true);
  win.loadFile('mode3.html');
  return win;
}

function switchMode(mode) {
  console.log('[Electron] switchMode:', mode);
  
  if (mode === 'main') {
    if (windows.mode1) { windows.mode1.close(); windows.mode1 = null; }
    if (windows.mode2) { windows.mode2.close(); windows.mode2 = null; }
    if (windows.mode3) { windows.mode3.close(); windows.mode3 = null; }
    if (windows.main) {
      windows.main.show();
      windows.main.focus();
    }
  } else if (mode === 'mode1') {
    if (windows.main) windows.main.hide();
    if (windows.mode2) windows.mode2.hide();
    if (windows.mode3) windows.mode3.hide();
    if (!windows.mode1) {
      windows.mode1 = createMode1Window();
      windows.mode1.show();
      windows.mode1.focus();
    } else {
      windows.mode1.show();
      windows.mode1.focus();
    }
  } else if (mode === 'mode2') {
    if (windows.main) windows.main.hide();
    if (windows.mode1) windows.mode1.hide();
    if (windows.mode3) windows.mode3.hide();
    if (!windows.mode2) {
      windows.mode2 = createMode2Window();
    } else {
      windows.mode2.show();
      windows.mode2.focus();
    }
  } else if (mode === 'mode3') {
    if (windows.main) windows.main.hide();
    if (windows.mode1) windows.mode1.hide();
    if (windows.mode2) windows.mode2.hide();
    if (!windows.mode3) {
      windows.mode3 = createMode3Window();
    } else {
      windows.mode3.show();
      windows.mode3.focus();
    }
  }
}

ipcMain.on('switch-mode', (event, mode) => {
  switchMode(mode);
});

app.whenReady().then(() => {
  windows.main = createMainWindow();
  windows.main.show();

  const contextMenu = Menu.buildFromTemplate([
    { label: '主页面', click: () => switchMode('main') },
    { label: '模式1', click: () => switchMode('mode1') },
    { label: '模式2', click: () => switchMode('mode2') },
    { label: '模式3', click: () => switchMode('mode3') },
    { type: 'separator' },
    { label: '显示窗口', click: () => { if (windows.main) { windows.main.show(); windows.main.focus(); } } },
    { label: '隐藏窗口', click: () => { if (windows.main) windows.main.hide(); } },
    { type: 'separator' },
    { label: '退出', click: () => app.exit(0) }
  ]);

  const iconPath = path.join(__dirname, 'assets', 'icon.png');
  tray = new Tray(iconPath);
  tray.setToolTip('Hole Eye');
  tray.setContextMenu(contextMenu);
  
  tray.on('click', () => {
    if (windows.main) {
      windows.main.show();
      windows.main.focus();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    windows.main = createMainWindow();
  }
});

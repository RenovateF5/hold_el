const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

window.addEventListener("DOMContentLoaded", async () => {
  if (!window.__TAURI__?.window) {
    return;
  }

  const win = window.__TAURI__.window;
  const appWindow = win.getCurrent?.() || win.getCurrentWebviewWindow?.() || win.appWindow;
  
  if (!appWindow) return;

  await appWindow.setIgnoreCursorEvents(true);

  try {
    const monitors = await appWindow.availableMonitors();
    if (monitors && monitors.length > 0) {
      const monitor = monitors[0];
      const pos = await monitor.position();
      const LogicalPosition = win.LogicalPosition;
      if (LogicalPosition) {
        await appWindow.setPosition(new LogicalPosition(pos.x, pos.y));
      } else {
        await appWindow.setPosition({ x: pos.x, y: pos.y });
      }
    }
  } catch (e) {
    console.log("设置窗口位置失败:", e);
  }
});

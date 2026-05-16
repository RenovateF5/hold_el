const { invoke } = window.__TAURI__.core;

let timer = null;
let countdown = null;

function moveBall() {
  const ball = document.getElementById('ball');
  if (!ball) {
    console.log("ball not found");
    return;
  }
  const maxX = window.innerWidth - 60;
  const maxY = window.innerHeight - 60;
  const x = Math.floor(Math.random() * maxX);
  const y = Math.floor(Math.random() * maxY);
  console.log("moveBall:", x, y, "window:", window.innerWidth, window.innerHeight);
  ball.style.left = x + 'px';
  ball.style.top = y + 'px';
  console.log("ball style after:", ball.style.cssText, "computed left:", getComputedStyle(ball).left);
}

function stopMode1() {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
  if (countdown) {
    clearTimeout(countdown);
    countdown = null;
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  console.log("mode1 DOMContentLoaded");
  
  if (!window.__TAURI__?.window) {
    console.log("no tauri window");
    return;
  }

  const win = window.__TAURI__.window;
  const appWindow = win.getCurrent?.() || win.getCurrentWebviewWindow?.() || win.appWindow;
  
  if (!appWindow) {
    console.log("no app window");
    return;
  }

  await appWindow.setIgnoreCursorEvents(true);

  const ball = document.getElementById('ball');
  console.log("ball:", ball, "window size:", window.innerWidth, window.innerHeight);
  console.log("ball initial style:", ball.style.cssText);
  console.log("ball computed left:", getComputedStyle(ball).left, "top:", getComputedStyle(ball).top);

  moveBall();
  timer = setInterval(moveBall, 2000);

  countdown = setTimeout(() => {
    stopMode1();
    invoke('switch_mode', { mode: 'main' });
  }, 60000);
});

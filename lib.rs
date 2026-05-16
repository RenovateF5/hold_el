use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, State, AppHandle, Emitter, WebviewWindow, Listener,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct ModePayload {
    mode: String,
}

struct AppState {
    app_handle: Arc<std::sync::Mutex<Option<AppHandle>>>,
}

#[tauri::command]
fn switch_mode(mode: String, state: State<'_, AppState>) {
    println!("[Rust] switch_mode: {}", mode);
    if let Some(handle) = state.app_handle.lock().unwrap().as_ref() {
        let _ = handle.emit("mode-changed", ModePayload { mode });
    }
}

fn create_mode1_window(app: &tauri::AppHandle) -> Result<WebviewWindow, Box<dyn std::error::Error>> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        "mode1",
        tauri::WebviewUrl::App("mode1.html".into()),
    )
    .title("")
    .inner_size(1920.0, 1080.0)
    .transparent(true)
    .decorations(false)
    .resizable(false)
    .always_on_top(true)
    .skip_taskbar(false)
    .maximized(true)
    .build()?;
    window.set_ignore_cursor_events(true)?;
    Ok(window)
}

fn create_mode2_window(app: &tauri::AppHandle) -> Result<WebviewWindow, Box<dyn std::error::Error>> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        "mode2",
        tauri::WebviewUrl::App("mode2.html".into()),
    )
    .title("")
    .inner_size(1920.0, 1080.0)
    .transparent(true)
    .decorations(false)
    .resizable(false)
    .always_on_top(true)
    .skip_taskbar(false)
    .maximized(true)
    .build()?;
    window.set_ignore_cursor_events(true)?;
    Ok(window)
}

fn create_mode3_window(app: &tauri::AppHandle) -> Result<WebviewWindow, Box<dyn std::error::Error>> {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        "mode3",
        tauri::WebviewUrl::App("mode3.html".into()),
    )
    .title("")
    .inner_size(1920.0, 1080.0)
    .transparent(true)
    .decorations(false)
    .resizable(false)
    .always_on_top(true)
    .skip_taskbar(false)
    .maximized(true)
    .build()?;
    window.set_ignore_cursor_events(true)?;
    Ok(window)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        app_handle: Arc::new(std::sync::Mutex::new(None)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![switch_mode])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_ignore_cursor_events(true)?;

            // 监听窗口关闭事件，阻止退出，改为隐藏
            let win_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    println!("[Rust] 主窗口关闭事件 - 隐藏而非退出");
                    api.prevent_close();
                    let _ = win_clone.hide();
                }
            });

            // 保存 app handle
            {
                let state: State<'_, AppState> = app.state();
                *state.app_handle.lock().unwrap() = Some(app.handle().clone());
                println!("[Rust] app_handle saved");
            }

            // 监听模式切换事件
            let app_handle = app.handle().clone();
            let app_handle_clone = app_handle.clone();
            app_handle.listen("mode-changed", move |event| {
                println!("[Rust] received mode-changed event");
                if let Ok(payload) = serde_json::from_str::<ModePayload>(&event.payload()) {
                    match payload.mode.as_str() {
                        "mode1" => {
                            if let Some(main) = app_handle_clone.get_webview_window("main") {
                                let _ = main.hide();
                            }
                            if let Some(mode2) = app_handle_clone.get_webview_window("mode2") {
                                let _ = mode2.hide();
                            }
                            if let Some(mode3) = app_handle_clone.get_webview_window("mode3") {
                                let _ = mode3.hide();
                            }
                            if app_handle_clone.get_webview_window("mode1").is_none() {
                                if let Ok(mode1) = create_mode1_window(&app_handle_clone) {
                                    let _ = mode1.show();
                                    let _ = mode1.set_focus();
                                }
                            } else if let Some(mode1) = app_handle_clone.get_webview_window("mode1") {
                                let _ = mode1.show();
                                let _ = mode1.set_focus();
                            }
                        }
                        "mode2" => {
                            if let Some(main) = app_handle_clone.get_webview_window("main") {
                                let _ = main.hide();
                            }
                            if let Some(mode1) = app_handle_clone.get_webview_window("mode1") {
                                let _ = mode1.hide();
                            }
                            if let Some(mode3) = app_handle_clone.get_webview_window("mode3") {
                                let _ = mode3.hide();
                            }
                            if app_handle_clone.get_webview_window("mode2").is_none() {
                                if let Ok(mode2) = create_mode2_window(&app_handle_clone) {
                                    let _ = mode2.show();
                                    let _ = mode2.set_focus();
                                }
                            } else if let Some(mode2) = app_handle_clone.get_webview_window("mode2") {
                                let _ = mode2.show();
                                let _ = mode2.set_focus();
                            }
                        }
                        "mode3" => {
                            if let Some(main) = app_handle_clone.get_webview_window("main") {
                                let _ = main.hide();
                            }
                            if let Some(mode1) = app_handle_clone.get_webview_window("mode1") {
                                let _ = mode1.hide();
                            }
                            if let Some(mode2) = app_handle_clone.get_webview_window("mode2") {
                                let _ = mode2.hide();
                            }
                            if app_handle_clone.get_webview_window("mode3").is_none() {
                                if let Ok(mode3) = create_mode3_window(&app_handle_clone) {
                                    let _ = mode3.show();
                                    let _ = mode3.set_focus();
                                }
                            } else if let Some(mode3) = app_handle_clone.get_webview_window("mode3") {
                                let _ = mode3.show();
                                let _ = mode3.set_focus();
                            }
                        }
                        "main" => {
                            if let Some(mode1) = app_handle_clone.get_webview_window("mode1") {
                                let _ = mode1.close();
                            }
                            if let Some(mode2) = app_handle_clone.get_webview_window("mode2") {
                                let _ = mode2.close();
                            }
                            if let Some(mode3) = app_handle_clone.get_webview_window("mode3") {
                                let _ = mode3.close();
                            }
                            if let Some(main) = app_handle_clone.get_webview_window("main") {
                                let _ = main.show();
                                let _ = main.set_focus();
                            }
                        }
                        _ => {}
                    }
                }
            });

            // 创建系统托盘菜单
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "隐藏窗口", true, None::<&str>)?;
            let main_page = MenuItem::with_id(app, "main_page", "主页面", true, None::<&str>)?;
            let mode1 = MenuItem::with_id(app, "mode1", "模式1", true, None::<&str>)?;
            let mode2 = MenuItem::with_id(app, "mode2", "模式2", true, None::<&str>)?;
            let mode3 = MenuItem::with_id(app, "mode3", "模式3", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&main_page, &mode1, &mode2, &mode3, &show, &hide, &quit])?;

            // 创建系统托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
                    }
                    "main_page" => {
                        if let Some(mode1) = app.get_webview_window("mode1") {
                            let _ = mode1.close();
                        }
                        if let Some(main) = app.get_webview_window("main") {
                            let _ = main.show();
                            let _ = main.set_focus();
                        }
                    }
                    "mode1" => {
                        if let Some(handle) = app.app_handle().state::<AppState>().app_handle.lock().unwrap().as_ref() {
                            let _ = handle.emit("mode-changed", ModePayload { mode: "mode1".to_string() });
                        }
                    }
                    "mode2" => {
                        if let Some(handle) = app.app_handle().state::<AppState>().app_handle.lock().unwrap().as_ref() {
                            let _ = handle.emit("mode-changed", ModePayload { mode: "mode2".to_string() });
                        }
                    }
                    "mode3" => {
                        if let Some(handle) = app.app_handle().state::<AppState>().app_handle.lock().unwrap().as_ref() {
                            let _ = handle.emit("mode-changed", ModePayload { mode: "mode3".to_string() });
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

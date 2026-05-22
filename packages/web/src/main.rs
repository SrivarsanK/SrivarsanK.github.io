// Web Entry Point for Srivarsan Portfolio
use dioxus::prelude::*;
use ui::{BootSequence, LoginScreen, Taskbar, DesktopIcons, Terminal};

const FAVICON: Asset = asset!("/assets/favicon.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FRIEREN_BG: Asset = asset!("/assets/frieren.jpg");
const GARGANTUA_BG: Asset = asset!("/assets/gargantua.jpg");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy, PartialEq)]
enum OsState {
    Booting,
    Login,
    Desktop,
}

#[component]
fn App() -> Element {
    let mut os_state = use_signal(|| OsState::Booting);
    let mut current_theme = use_signal(|| "powershell".to_string());
    let mut wallpaper = use_signal(|| None::<String>);
    let mut external_command = use_signal(|| None::<String>);
    let is_minimized = use_signal(|| false);
    let frieren_str = FRIEREN_BG.to_string();
    let gargantua_str = GARGANTUA_BG.to_string();

    // Load saved preferences on client-side mount
    use_effect(move || {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                if let Ok(Some(saved_wallpaper)) = storage.get_item("desktop-bg") {
                    if saved_wallpaper == "/assets/frieren.jpg" {
                        wallpaper.set(Some(FRIEREN_BG.to_string()));
                    } else if saved_wallpaper == "/assets/gargantua.jpg" {
                        wallpaper.set(Some(GARGANTUA_BG.to_string()));
                    } else {
                        wallpaper.set(Some(saved_wallpaper));
                    }
                } else {
                    // Default wallpaper: Gargantua
                    wallpaper.set(Some(GARGANTUA_BG.to_string()));
                }
                if let Ok(Some(saved_theme)) = storage.get_item("terminal-theme") {
                    current_theme.set(saved_theme);
                }
            }
        }
    });

    let handle_reset = move |_| {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                let _ = storage.clear();
            }
            let _ = win.location().reload();
        }
    };

    let desktop_style = match wallpaper() {
        Some(w) => format!("background-image: url('{}'); background-size: cover; background-position: center; background-repeat: no-repeat;", w),
        None => "background-image: none; background-color: #000;".to_string(),
    };

    rsx! {
        document::Title { "Srivarsan | Portfolio" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        match os_state() {
            OsState::Booting => {
                rsx! {
                    BootSequence {
                        on_complete: move |_| os_state.set(OsState::Login)
                    }
                }
            }
            OsState::Login => {
                rsx! {
                    LoginScreen {
                        on_login: move |_| os_state.set(OsState::Desktop),
                        wallpaper: wallpaper()
                    }
                }
            }
            OsState::Desktop => {
                rsx! {
                    div {
                        class: "h-screen text-foreground relative overflow-hidden desktop-enter",
                        style: "{desktop_style}",
                        
                        // Background Overlay
                        if wallpaper().is_some() {
                            div { style: "position: absolute; inset: 0; background: rgba(0,0,0,0.35); z-index: 0;" }
                        }

                        // Terminal — full-viewport positioned layer, z-index 30
                        // This MUST be a full-size absolute container so the terminal's own
                        // absolute + transform positioning works relative to the viewport
                        div {
                            style: "position: absolute; top: 0; left: 0; right: 0; bottom: 40px; z-index: 30; pointer-events: none;",
                            Terminal {
                                external_command,
                                current_theme: current_theme.clone(),
                                is_minimized,
                            }
                        }

                        // Desktop Icons Layer — z-index 20, above terminal, pointer-events own
                        div {
                            style: "position: absolute; top: 0; left: 0; right: 0; bottom: 40px; z-index: 20; pointer-events: none;",
                            DesktopIcons {
                                on_icon_click: move |cmd: String| {
                                    external_command.set(Some(cmd));
                                }
                            }
                        }

                        // Bottom Taskbar — z-index: 50
                        Taskbar {
                            current_theme,
                            wallpaper,
                            default_wallpaper: gargantua_str.clone(),
                            frieren_wallpaper: frieren_str.clone(),
                            is_minimized,
                            on_logout: move |_| os_state.set(OsState::Login),
                            on_reset: handle_reset
                        }
                    }
                }
            }
        }
    }
}

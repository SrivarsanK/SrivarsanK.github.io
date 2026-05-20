use dioxus::prelude::*;
use ui::{BootSequence, LoginScreen, Taskbar, DesktopIcons, Terminal};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const FRIEREN_BG: Asset = asset!("/assets/frieren.jpg");

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

    // Load saved preferences on client-side mount
    use_effect(move || {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                if let Ok(Some(saved_wallpaper)) = storage.get_item("terminal-wallpaper") {
                    wallpaper.set(Some(saved_wallpaper));
                } else {
                    // Default wallpaper as asset URL
                    wallpaper.set(Some(FRIEREN_BG.to_string()));
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
        Some(w) => format!("background-image: url({}); background-size: cover; background-position: center; background-repeat: no-repeat;", w),
        None => "background-color: #000;".to_string(),
    };

    rsx! {
        // Global app resources
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
                        class: "h-screen text-foreground relative overflow-hidden",
                        style: "{desktop_style}",
                        
                        // Background Overlay if wallpaper is set
                        if wallpaper().is_some() {
                            div { class: "absolute inset-0 bg-black/40", style: "z-index: 0;" }
                        }

                        // Desktop Icons Layer — sits on top, z-index: 20
                        div {
                            style: "position: absolute; top: 0; left: 0; right: 0; bottom: 40px; z-index: 20;",
                            DesktopIcons {
                                on_icon_click: move |cmd: String| {
                                    external_command.set(Some(cmd));
                                }
                            }
                        }

                        // Terminal Layer — z-index: 10, fills space above taskbar
                        div {
                            style: "position: absolute; top: 0; left: 0; right: 0; bottom: 40px; z-index: 10; display: flex; align-items: center; justify-content: center; padding: 1rem; pointer-events: none;",
                            div {
                                style: "width: 100%; max-width: 56rem; height: 100%; display: flex; align-items: center; justify-content: center; pointer-events: auto;",
                                Terminal {
                                    external_command,
                                    current_theme: current_theme.clone(),
                                }
                            }
                        }

                        // Bottom Taskbar — fixed at bottom, z-index: 50
                        Taskbar {
                            current_theme,
                            wallpaper,
                            on_logout: move |_| os_state.set(OsState::Login),
                            on_reset: handle_reset
                        }
                    }
                }
            }
        }
    }
}

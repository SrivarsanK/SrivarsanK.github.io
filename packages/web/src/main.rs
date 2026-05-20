use dioxus::prelude::*;
use ui::{BootSequence, LoginScreen, Taskbar};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

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

    // Load saved preferences on client-side mount
    use_effect(move || {
        if let Some(win) = web_sys::window() {
            if let Ok(Some(storage)) = win.local_storage() {
                if let Ok(Some(saved_wallpaper)) = storage.get_item("terminal-wallpaper") {
                    wallpaper.set(Some(saved_wallpaper));
                } else {
                    wallpaper.set(Some("/frieren.jpg".to_string()));
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
                        class: "h-screen text-foreground relative overflow-hidden animate-in fade-in duration-1000",
                        style: "{desktop_style}",
                        
                        // Background Overlay if wallpaper is set
                        if wallpaper().is_some() {
                            div { class: "absolute inset-0 bg-black/40 z-0" }
                        }

                        // Main Desktop Content
                        main {
                            class: "h-[calc(100vh-40px)] relative flex items-center justify-center p-4 overflow-hidden z-10",
                            div {
                                class: "text-terminal-green text-xl terminal-glow",
                                "Desktop Area (Current Theme: {current_theme})"
                            }
                        }

                        // Bottom Taskbar
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

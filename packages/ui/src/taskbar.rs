use dioxus::prelude::*;
use chrono::Local;
use gloo_timers::future::sleep;
use std::time::Duration;
use wasm_bindgen::JsCast;

#[derive(Props, Clone, PartialEq)]
pub struct TaskbarProps {
    current_theme: Signal<String>,
    wallpaper: Signal<Option<String>>,
    default_wallpaper: String,
    is_minimized: Signal<bool>,
    on_logout: EventHandler<()>,
    on_reset: EventHandler<()>,
}

struct ThemeInfo {
    name: &'static str,
    color: &'static str,
    id: &'static str,
}

const THEMES: &[ThemeInfo] = &[
    ThemeInfo { name: "PowerShell", color: "#012456", id: "powershell" },
    ThemeInfo { name: "Classic CMD", color: "#000000", id: "cmd" },
    ThemeInfo { name: "Matrix", color: "#000500", id: "matrix" },
    ThemeInfo { name: "Ubuntu", color: "#300a24", id: "ubuntu" },
    ThemeInfo { name: "Dracula", color: "#282a36", id: "dracula" },
];

#[component]
pub fn Taskbar(props: TaskbarProps) -> Element {
    let mut is_start_open = use_signal(|| false);
    let mut is_theme_open = use_signal(|| false);
    let mut is_wallpaper_open = use_signal(|| false);
    let mut is_reset_open = use_signal(|| false);
    let mut time = use_signal(|| Local::now());
    let mut is_minimized = props.is_minimized;

    // Clock update loop
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            time.set(Local::now());
        }
    });

    let time_val = time();
    let time_str = time_val.format("%-I:%M %P").to_string();
    let date_str = time_val.format("%-d/%-m/%Y").to_string();

    let mut wallpaper = props.wallpaper;
    let mut current_theme = props.current_theme;

    let handle_wallpaper_change = move |_e| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let file_input = document
            .get_element_by_id("wallpaper-upload-input")
            .unwrap()
            .dyn_into::<web_sys::HtmlInputElement>()
            .unwrap();
        if let Some(files) = file_input.files() {
            if let Some(file) = files.get(0) {
                let reader = web_sys::FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_e: web_sys::Event| {
                    if let Ok(result) = reader_clone.result() {
                        if let Some(str_val) = result.as_string() {
                            wallpaper.set(Some(str_val.clone()));
                            if let Some(win) = web_sys::window() {
                                if let Ok(Some(storage)) = win.local_storage() {
                                    let _ = storage.set_item("terminal-wallpaper", &str_val);
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                reader.set_onload(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
                let _ = reader.read_as_data_url(&file);
            }
        }
    };

    rsx! {
        footer {
            class: "fixed bottom-0 left-0 right-0 h-[40px] bg-[#00122e]/80 backdrop-blur-md border-t border-white/10 z-50 flex items-center justify-between px-2 select-none",
            
            // Left Side: Start Button & Terminal Icon
            div {
                class: "flex items-center gap-1 h-full",
                
                // Start Button & Start Menu
                div {
                    class: "relative flex items-center h-full",
                    button {
                        onclick: move |_| is_start_open.toggle(),
                        style: "background: transparent; border: none; padding: 0; outline: none; cursor: pointer; display: flex; align-items: center; justify-content: center; width: 48px; height: 40px; transition: background-color 0.2s;",
                        class: if is_start_open() { "bg-white/20 btn-press" } else { "hover:bg-white/10 btn-press" },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            fill: "#60a5fa",
                            stroke: "#60a5fa",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect { x: "3", y: "3", width: "7", height: "7", rx: "1.5" }
                            rect { x: "14", y: "3", width: "7", height: "7", rx: "1.5" }
                            rect { x: "14", y: "14", width: "7", height: "7", rx: "1.5" }
                            rect { x: "3", y: "14", width: "7", height: "7", rx: "1.5" }
                        }
                    }

                    // Start Menu Popup
                    if is_start_open() {
                        div {
                            class: "menu-slide-up",
                            style: "position: absolute; bottom: 44px; left: 0; width: 300px; background-color: rgba(28, 28, 28, 0.95); backdrop-filter: blur(24px); -webkit-backdrop-filter: blur(24px); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 0.5rem; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5); overflow: hidden; display: flex; flex-direction: column; z-index: 100;",
                            
                            // User Info Card
                            div {
                                style: "padding: 1rem; border-bottom: 1px solid rgba(255,255,255,0.05); display: flex; align-items: center;",
                                div {
                                    style: "display: flex; align-items: center; gap: 0.75rem; padding: 0.5rem; border-radius: 0.375rem; cursor: default; width: 100%; text-align: left;",
                                    div {
                                        style: "width: 2.5rem; height: 2.5rem; border-radius: 50%; background: linear-gradient(135deg, #6e6aca, #8b5cf6); display: flex; align-items: center; justify-content: center; font-weight: 700; color: #fff; font-size: 1rem; flex-shrink: 0;",
                                        "R"
                                    }
                                    div {
                                        style: "display: flex; flex-direction: column; text-align: left;",
                                        div { style: "font-size: 0.875rem; font-weight: 600; color: #fff; line-height: 1.25;", "Ovi ren" }
                                        div { style: "font-size: 0.7rem; color: rgba(255,255,255,0.45); line-height: 1.4; margin-top: 0.125rem;", "Writer • Script Author" }
                                    }
                                }
                            }

                            // Pinned Section
                            div {
                                style: "padding: 0.5rem; display: flex; flex-direction: column; gap: 0.25rem;",
                                div {
                                    style: "padding: 0.5rem 0.75rem; font-size: 11px; font-weight: 600; color: rgba(255,255,255,0.3); text-transform: uppercase; letter-spacing: 0.05em; text-align: left;",
                                    "Pinned"
                                }
                                
                                a {
                                    href: "https://github.com",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    style: "display: flex; align-items: center; gap: 0.75rem; padding: 0.75rem 1rem; border-radius: 0.375rem; transition: background-color 0.15s; text-decoration: none; color: #fff;",
                                    class: "hover:bg-white/10 text-white/80 hover:text-white group",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "20",
                                        height: "20",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        class: "group-hover:scale-110 transition-transform",
                                        path { d: "M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" }
                                        path { d: "M9 18c-4.51 2-5-2-7-2" }
                                    }
                                    span { style: "font-size: 0.875rem;", "GitHub" }
                                }

                                a {
                                    href: "mailto:hello@developer.dev",
                                    style: "display: flex; align-items: center; gap: 0.75rem; padding: 0.75rem 1rem; border-radius: 0.375rem; transition: background-color 0.15s; text-decoration: none; color: #fff;",
                                    class: "hover:bg-white/10 text-white/80 hover:text-white group",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "20",
                                        height: "20",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        class: "group-hover:scale-110 transition-transform",
                                        rect { width: "20", height: "16", x: "2", y: "4", rx: "2" }
                                        path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
                                    }
                                    span { style: "font-size: 0.875rem;", "Email" }
                                }
                            }

                            // Start Menu Footer
                            div {
                                style: "padding: 0.75rem 1rem; background: rgba(0,0,0,0.2); display: flex; align-items: center; justify-content: space-between; flex-direction: row; margin-top: auto;",
                                div {
                                    onclick: move |_| {
                                        is_start_open.set(false);
                                        props.on_logout.call(());
                                    },
                                    style: "display: flex; align-items: center; gap: 0.4rem; font-size: 0.7rem; color: rgba(255,255,255,0.5); cursor: pointer; transition: color 0.15s;",
                                    class: "hover:text-white group",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "12",
                                        height: "12",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" }
                                        polyline { points: "16 17 21 12 16 7" }
                                        line { x1: "21", y1: "12", x2: "9", y2: "12" }
                                    }
                                    span { "Log out" }
                                }
                                span {
                                    onclick: move |_| {
                                        is_start_open.set(false);
                                        is_reset_open.set(true);
                                    },
                                    style: "font-size: 0.7rem; color: rgba(255,255,255,0.5); cursor: pointer; font-weight: 600; transition: color 0.15s;",
                                    class: "hover:text-red-400",
                                    "Reset"
                                }
                            }
                        }
                    }
                }

                // Pinned Terminal Icon (sticks right of Start Button)
                div {
                    class: "w-10 h-10 rounded-md hover:bg-white/10 transition-colors flex items-center justify-center cursor-pointer relative group",
                    onclick: move |_| is_minimized.toggle(),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "20",
                        height: "20",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        class: "text-blue-400 drop-shadow-[0_0_4px_rgba(96,165,250,0.5)]",
                        polyline { points: "4 17 10 11 4 5" }
                        line { x1: "12", y1: "19", x2: "20", y2: "19" }
                    }
                    div {
                        class: "absolute bottom-0 w-3 h-1 rounded-t-sm bg-blue-400 transition-all duration-300",
                    }
                }
            }

            // Right Side: System Tray & Clock
            div {
                class: "flex items-center justify-end gap-1 h-full px-2",
                
                // Wallpaper Dropdown Menu
                div {
                    class: "relative flex items-center h-full",
                    button {
                        onclick: move |_| {
                            is_wallpaper_open.toggle();
                            is_theme_open.set(false);
                            is_start_open.set(false);
                        },
                        style: "background: transparent; border: none; outline: none; cursor: pointer; display: flex; align-items: center; justify-content: center;",
                        class: "p-2 hover:bg-white/10 rounded-md transition-colors",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "text-white/70 hover:text-white",
                            rect { width: "18", height: "18", x: "3", y: "3", rx: "2", ry: "2" }
                            circle { cx: "9", cy: "9", r: "2" }
                            path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" }
                        }
                    }

                    if is_wallpaper_open() {
                        div {
                            class: "menu-slide-up",
                            style: "position: absolute; bottom: 44px; right: 0; width: 160px; background-color: rgba(28, 28, 28, 0.95); backdrop-filter: blur(24px); -webkit-backdrop-filter: blur(24px); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 0.5rem; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5); overflow: hidden; padding: 0.25rem 0; display: flex; flex-direction: column; z-index: 100;",
                            div {
                                onclick: move |_| {
                                    let bg_url = props.default_wallpaper.clone();
                                    wallpaper.set(Some(bg_url.clone()));
                                    if let Some(win) = web_sys::window() {
                                        if let Ok(Some(storage)) = win.local_storage() {
                                            let _ = storage.set_item("terminal-wallpaper", &bg_url);
                                        }
                                    }
                                    is_wallpaper_open.set(false);
                                },
                                style: "display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0.75rem; cursor: pointer; color: #fff; transition: background-color 0.15s; font-size: 0.75rem; text-align: left;",
                                class: "hover:bg-white/10",
                                span { "Default (Frieren)" }
                            }
                            div {
                                onclick: move |_| {
                                    wallpaper.set(None);
                                    if let Some(win) = web_sys::window() {
                                        if let Ok(Some(storage)) = win.local_storage() {
                                            let _ = storage.remove_item("terminal-wallpaper");
                                        }
                                    }
                                    is_wallpaper_open.set(false);
                                },
                                style: "display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0.75rem; cursor: pointer; color: #fff; transition: background-color 0.15s; font-size: 0.75rem; text-align: left;",
                                class: "hover:bg-white/10",
                                span { "None (Dark)" }
                            }
                            label {
                                style: "display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0.75rem; cursor: pointer; color: #fff; transition: background-color 0.15s; font-size: 0.75rem; text-align: left;",
                                class: "hover:bg-white/10",
                                span { "Upload Custom..." }
                                input {
                                    id: "wallpaper-upload-input",
                                    type: "file",
                                    class: "hidden",
                                    accept: "image/*",
                                    onchange: handle_wallpaper_change
                                }
                            }
                        }
                    }
                }

                // Theme Dropdown Menu
                div {
                    class: "relative flex items-center h-full",
                    button {
                        onclick: move |_| {
                            is_theme_open.toggle();
                            is_wallpaper_open.set(false);
                            is_start_open.set(false);
                        },
                        style: "background: transparent; border: none; outline: none; cursor: pointer; display: flex; align-items: center; justify-content: center;",
                        class: "p-2 hover:bg-white/10 rounded-md transition-colors",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "text-white/70 hover:text-white",
                            circle { cx: "13.5", cy: "6.5", r: ".5", fill: "currentColor" }
                            circle { cx: "17.5", cy: "10.5", r: ".5", fill: "currentColor" }
                            circle { cx: "8.5", cy: "7.5", r: ".5", fill: "currentColor" }
                            circle { cx: "6.5", cy: "12.5", r: ".5", fill: "currentColor" }
                            path { d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.836-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" }
                        }
                    }

                    if is_theme_open() {
                        div {
                            class: "menu-slide-up",
                            style: "position: absolute; bottom: 44px; right: 0; width: 150px; background-color: rgba(28, 28, 28, 0.95); backdrop-filter: blur(24px); -webkit-backdrop-filter: blur(24px); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 0.5rem; box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5); overflow: hidden; padding: 0.25rem 0; display: flex; flex-direction: column; z-index: 100;",
                            for theme in THEMES {
                                div {
                                    key: "{theme.id}",
                                    onclick: move |_| {
                                        current_theme.set(theme.id.to_string());
                                        if let Some(win) = web_sys::window() {
                                            if let Ok(Some(storage)) = win.local_storage() {
                                                let _ = storage.set_item("terminal-theme", theme.id);
                                            }
                                        }
                                        is_theme_open.set(false);
                                    },
                                    style: "display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0.75rem; cursor: pointer; color: #fff; transition: background-color 0.15s; font-size: 0.75rem; text-align: left;",
                                    class: "hover:bg-white/10",
                                    div {
                                        style: "width: 0.75rem; height: 0.75rem; border-radius: 50%; border: 1px solid rgba(255,255,255,0.2); flex-shrink: 0; background-color: {theme.color}"
                                    }
                                    span { "{theme.name}" }
                                }
                            }
                        }
                    }
                }

                // Separator
                div { class: "w-px h-5 bg-white/10 mx-2" }

                // Clock
                div {
                    class: "text-[11px] font-mono text-white/70 px-2 select-none text-right font-medium leading-tight",
                    div { "{time_str}" }
                    div { "{date_str}" }
                }
            }
        }

        // Reset Confirmation Dialog
        if is_reset_open() {
            div {
                class: "fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm backdrop-fade-in",
                div {
                    class: "bg-[#1c1c1c]/95 border border-white/10 text-white rounded-lg p-6 max-w-md w-full shadow-2xl menu-slide-up mx-4",
                    h2 { class: "text-lg font-bold mb-2", "Are you absolutely sure?" }
                    p {
                        class: "text-sm text-white/60 mb-6 leading-relaxed",
                        "This action will reset your wallpaper, theme, and terminal settings to their default values. The page will reload."
                    }
                    div {
                        class: "flex justify-end gap-3",
                        button {
                            onclick: move |_| is_reset_open.set(false),
                            class: "px-4 py-2 text-sm bg-transparent hover:bg-white/5 border border-white/10 rounded transition-colors text-white",
                            "Cancel"
                        }
                        button {
                            onclick: move |_| {
                                is_reset_open.set(false);
                                props.on_reset.call(());
                            },
                            class: "px-4 py-2 text-sm bg-red-500 hover:bg-red-600 rounded transition-colors text-white",
                            "Reset Everything"
                        }
                    }
                }
            }
        }
    }
}

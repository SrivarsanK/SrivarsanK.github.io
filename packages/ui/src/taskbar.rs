use dioxus::prelude::*;
use chrono::Local;
use gloo_timers::future::sleep;
use std::time::Duration;
use wasm_bindgen::JsCast;

#[derive(Props, Clone, PartialEq)]
pub struct TaskbarProps {
    current_theme: Signal<String>,
    wallpaper: Signal<Option<String>>,
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
    let mut is_reset_open = use_signal(|| false);
    let mut time = use_signal(|| Local::now());

    // Clock update loop
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            time.set(Local::now());
        }
    });

    let time_val = time();
    let time_str = time_val.format("%H:%M").to_string();
    let date_str = time_val.format("%Y-%m-%d").to_string();

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
            
            // Left Side: Start Button
            div {
                class: "flex items-center",
                div {
                    class: "relative",
                    button {
                        onclick: move |_| is_start_open.toggle(),
                        class: if is_start_open() { "flex items-center justify-center w-[48px] h-[40px] hover:bg-white/10 transition-colors bg-white/20" } else { "flex items-center justify-center w-[48px] h-[40px] hover:bg-white/10 transition-colors" },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "text-blue-400 fill-blue-400",
                            rect { x: "3", y: "3", width: "7", height: "7" }
                            rect { x: "14", y: "3", width: "7", height: "7" }
                            rect { x: "14", y: "14", width: "7", height: "7" }
                            rect { x: "3", y: "14", width: "7", height: "7" }
                        }
                    }

                    // Start Menu Popup
                    if is_start_open() {
                        div {
                            class: "absolute bottom-[48px] left-0 w-[300px] bg-[#1c1c1c]/95 backdrop-blur-xl border border-white/10 rounded-lg shadow-2xl overflow-hidden animate-in slide-in-from-bottom-2 duration-200 z-50",
                            
                            // User Info Card
                            div {
                                class: "p-4 border-b border-white/5",
                                div {
                                    class: "flex items-center gap-3 px-2 py-3 rounded-md hover:bg-white/5 transition-colors cursor-default",
                                    div {
                                        class: "w-10 h-10 rounded-full bg-gradient-to-br from-blue-500 to-purple-500 flex items-center justify-center font-bold text-white",
                                        "O"
                                    }
                                    div {
                                        div { class: "text-sm font-semibold text-white", "Ovi ren" }
                                        div { class: "text-[11px] text-white/50", "Writer • Script Author" }
                                    }
                                }
                            }

                            // Pinned Section
                            div {
                                class: "p-2 space-y-1",
                                div { class: "px-3 py-2 text-[11px] font-semibold text-white/30 uppercase tracking-wider", "Pinned" }
                                
                                a {
                                    href: "https://github.com",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "flex items-center gap-3 px-4 py-3 rounded-md hover:bg-white/10 transition-colors text-white/80 hover:text-white group",
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
                                    span { class: "text-sm", "GitHub" }
                                }

                                a {
                                    href: "mailto:hello@developer.dev",
                                    class: "flex items-center gap-3 px-4 py-3 rounded-md hover:bg-white/10 transition-colors text-white/80 hover:text-white group",
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
                                    span { class: "text-sm", "Email" }
                                }
                            }

                            // Start Menu Footer
                            div {
                                class: "mt-4 p-4 bg-black/20 flex items-center justify-between text-[11px] text-white/40",
                                div {
                                    onclick: move |_| {
                                        is_start_open.set(false);
                                        props.on_logout.call(());
                                    },
                                    class: "flex items-center gap-2 hover:text-white cursor-pointer transition-colors group",
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
                                        class: "group-hover:scale-110 transition-transform",
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
                                    class: "hover:text-red-400 cursor-pointer transition-colors font-semibold",
                                    "Reset"
                                }
                            }
                        }
                    }
                }
            }

            // Right Side: System Tray & Clock
            div {
                class: "flex items-center gap-1 h-full px-2",
                
                // Wallpaper Upload Button
                label {
                    class: "cursor-pointer p-2 hover:bg-white/10 rounded-md transition-colors flex items-center justify-center",
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
                    input {
                        id: "wallpaper-upload-input",
                        type: "file",
                        class: "hidden",
                        accept: "image/*",
                        onchange: handle_wallpaper_change
                    }
                }

                // Theme Dropdown Menu
                div {
                    class: "relative",
                    button {
                        onclick: move |_| is_theme_open.toggle(),
                        class: "p-2 hover:bg-white/10 rounded-md transition-colors flex items-center justify-center",
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
                            path { d: "M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 14.7255 3.09032 17.1962 4.85857 19C5.03444 19.1759 5.09914 19.431 5.02102 19.6678C4.78912 20.3705 4.66667 21.171 4.66667 22C4.66667 22 7.5 22 12 22Z" }
                        }
                    }

                    if is_theme_open() {
                        div {
                            class: "absolute bottom-[48px] right-0 w-[150px] bg-[#1c1c1c]/95 backdrop-blur-xl border border-white/10 rounded-lg shadow-2xl overflow-hidden py-1 z-50",
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
                                    class: "flex items-center gap-2 px-3 py-2 cursor-pointer hover:bg-white/10 text-white transition-colors text-xs",
                                    div {
                                        class: "w-3 h-3 rounded-full border border-white/20",
                                        style: "background-color: {theme.color}"
                                    }
                                    span { "{theme.name}" }
                                }
                            }
                        }
                    }
                }

                // Separator
                div { class: "w-px h-6 bg-white/10 mx-2" }

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
                class: "fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm",
                div {
                    class: "bg-[#1c1c1c]/95 border border-white/10 text-white rounded-lg p-6 max-w-md w-full shadow-2xl animate-in zoom-in-95 duration-200 mx-4",
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

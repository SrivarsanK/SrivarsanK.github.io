use dioxus::prelude::*;
use chrono::Local;
use gloo_timers::future::sleep;
use std::time::Duration;

#[derive(Props, Clone, PartialEq)]
pub struct LoginScreenProps {
    on_login: EventHandler<()>,
    #[props(default = None)]
    wallpaper: Option<String>,
}

#[component]
pub fn LoginScreen(props: LoginScreenProps) -> Element {
    let mut is_logging_in = use_signal(|| false);
    let mut time = use_signal(|| Local::now());

    // Clock update loop
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        loop {
            sleep(Duration::from_millis(1000)).await;
            time.set(Local::now());
        }
    });

    let handle_login = move |_| {
        is_logging_in.set(true);
        spawn(async move {
            sleep(Duration::from_millis(1500)).await;
            props.on_login.call(());
        });
    };

    let time_val = time();
    let time_str = time_val.format("%H:%M").to_string();
    let date_str = time_val.format("%A, %B %-d").to_string();

    let bg_style = match &props.wallpaper {
        Some(w) => format!("background-image: url({}); background-size: cover; background-position: center;", w),
        None => "background-color: #000;".to_string(),
    };

    rsx! {
        div {
            class: "fixed inset-0 z-[40] flex items-center justify-center overflow-hidden",
            style: "{bg_style}",
            
            // Background Overlay
            div { class: "absolute inset-0 bg-black/30 backdrop-blur-[2px]" }

            // Clock Area
            div {
                class: "absolute bottom-12 left-12 text-white animate-in fade-in slide-in-from-bottom-8 duration-1000",
                div {
                    class: "text-7xl font-light mb-2",
                    "{time_str}"
                }
                div {
                    class: "text-xl font-medium opacity-80",
                    "{date_str}"
                }
            }

            // Login Card
            div {
                class: "relative z-10 flex flex-col items-center animate-in zoom-in-95 duration-500",
                div {
                    class: "w-48 h-48 rounded-full bg-white/10 backdrop-blur-md border border-white/20 flex items-center justify-center mb-6 overflow-hidden",
                    div {
                        class: "w-full h-full flex items-center justify-center bg-gradient-to-br from-blue-500/50 to-purple-500/50",
                        span {
                            class: "text-7xl font-bold text-white tracking-tighter shadow-xl",
                            "R"
                        }
                    }
                }
                
                h1 {
                    class: "text-3xl font-semibold text-white mb-2 drop-shadow-lg",
                    "Ren"
                }

                if is_logging_in() {
                    div {
                        class: "flex flex-col items-center gap-4 mt-4",
                        div { class: "w-8 h-8 border-4 border-white/30 border-t-white rounded-full animate-spin" }
                        span {
                            class: "text-white/80 text-sm font-medium animate-pulse",
                            "Logging in..."
                        }
                    }
                } else {
                    button {
                        onclick: handle_login,
                        class: "group mt-4 flex items-center gap-3 bg-white/10 hover:bg-white/20 backdrop-blur-md border border-white/20 px-8 py-3 rounded-md transition-all duration-300 transform hover:scale-105",
                        span {
                            class: "text-white font-medium",
                            "Login"
                        }
                        // Inline SVG for Login Icon
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
                            class: "text-white group-hover:translate-x-1 transition-transform",
                            path { d: "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" }
                            polyline { points: "10 17 15 12 10 7" }
                            line { x1: "15", y1: "12", x2: "3", y2: "12" }
                        }
                    }
                }
            }

            // Shutdown/Power buttons (Visual only)
            div {
                class: "absolute bottom-8 right-8 flex items-center gap-4 opacity-70",
                div {
                    class: "w-6 h-6 border border-white/20 rounded flex items-center justify-center text-white text-[10px]",
                    "Wi-Fi"
                }
                div {
                    class: "w-6 h-6 border border-white/20 rounded flex items-center justify-center text-white text-[10px]",
                    "EN"
                }
            }
        }
    }
}

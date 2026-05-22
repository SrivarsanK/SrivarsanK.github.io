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
    let mut is_exiting = use_signal(|| false);
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
            // Show spinner for a moment
            sleep(Duration::from_millis(800)).await;
            // Trigger exit animation
            is_exiting.set(true);
            // Wait for exit animation to finish
            sleep(Duration::from_millis(550)).await;
            props.on_login.call(());
        });
    };

    let time_val = time();
    let time_str = time_val.format("%-I:%M %P").to_string();
    let date_str = time_val.format("%A, %-d %B").to_string();

    let bg_style = match &props.wallpaper {
        Some(w) => format!(
            "background-image: url('{}'); background-size: cover; background-position: center;",
            w
        ),
        None => "background-image: none; background-color: #000;".to_string(),
    };

    let exit_class = if is_exiting() { "login-exit" } else { "" };

    rsx! {
        div {
            class: "fixed inset-0 z-[90] flex items-center justify-center overflow-hidden {exit_class}",
            style: "background-color: #000; {bg_style}",

            // Background Overlay — blur + darken
            div {
                style: "position: absolute; inset: 0; background: rgba(0,0,0,0.30); backdrop-filter: blur(2px); -webkit-backdrop-filter: blur(2px);"
            }

            // ── CLOCK — large, bottom-left with slide-up entrance ──
            div {
                class: "login-clock-enter",
                style: "position: absolute; bottom: 3rem; left: 3rem; color: white; z-index: 10;",
                div {
                    style: "font-size: 4.5rem; line-height: 1; font-weight: 300; margin-bottom: 0.5rem; font-family: ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';",
                    "{time_str}"
                }
                div {
                    style: "font-size: 1.25rem; font-weight: 500; opacity: 0.8; font-family: ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';",
                    "{date_str}"
                }
            }

            // ── LOGIN CARD — centered with orchestrated entrance ──
            div {
                style: "position: relative; z-index: 10; display: flex; flex-direction: column; align-items: center;",

                // Avatar circle with scale-bounce entrance
                div {
                    class: "login-avatar-enter",
                    style: "width: 12rem; height: 12rem; border-radius: 50%; overflow: hidden; margin-bottom: 1.5rem; border: 2px solid rgba(255,255,255,0.25); box-shadow: 0 25px 50px -12px rgba(0,0,0,0.5);",
                    div {
                        style: "width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, rgba(59,130,246,0.6), rgba(168,85,247,0.6)); backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px);",
                        span {
                            style: "font-size: 5rem; font-weight: 700; color: white; letter-spacing: -0.05em; text-shadow: 0 4px 20px rgba(0,0,0,0.3);",
                            "S"
                        }
                    }
                }

                // Username + Login button with delayed slide-up
                div {
                    class: "login-card-enter",
                    style: "display: flex; flex-direction: column; align-items: center;",

                    h1 {
                        style: "font-size: 1.875rem; font-weight: 600; color: white; margin-bottom: 0.5rem; text-shadow: 0 2px 10px rgba(0,0,0,0.3);",
                        "Srivarsan"
                    }

                    // Login button / spinner
                    if is_logging_in() {
                        div {
                            class: "animate-fade-in",
                            style: "display: flex; flex-direction: column; align-items: center; gap: 1rem; margin-top: 1rem;",
                            div {
                                style: "width: 2rem; height: 2rem; border: 4px solid rgba(255,255,255,0.3); border-top-color: white; border-radius: 50%;",
                                class: "animate-spin"
                            }
                            span {
                                style: "color: rgba(255,255,255,0.8); font-size: 0.875rem; font-weight: 500;",
                                class: "animate-pulse",
                                "Logging in..."
                            }
                        }
                    } else {
                        button {
                            onclick: handle_login,
                            class: "group hover:scale-105 active:scale-95",
                            style: "margin-top: 1.5rem; display: flex; align-items: center; gap: 0.75rem; background: linear-gradient(135deg, rgba(255,255,255,0.15), rgba(255,255,255,0.05)); backdrop-filter: blur(20px) saturate(160%); -webkit-backdrop-filter: blur(20px) saturate(160%); border: 1px solid rgba(255,255,255,0.2); border-top: 1px solid rgba(255,255,255,0.5); border-left: 1px solid rgba(255,255,255,0.4); box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3), inset 0 1px 2px rgba(255, 255, 255, 0.4); padding: 0.75rem 2.5rem; border-radius: 0.375rem; cursor: pointer; transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); color: white; font-weight: 600; font-size: 1rem; letter-spacing: 0.05em; text-shadow: 0 1px 2px rgba(0,0,0,0.2);",
                            span { "Login" }
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
                                path { d: "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" }
                                polyline { points: "10 17 15 12 10 7" }
                                line { x1: "15", y1: "12", x2: "3", y2: "12" }
                            }
                        }
                    }
                }
            }

            // ── WIFI + LANGUAGE — bottom-right with delayed fade-in ──
            div {
                class: "login-icons-enter",
                style: "position: absolute; bottom: 2rem; right: 2rem; display: flex; align-items: center; gap: 0.75rem; z-index: 10;",

                // Wi-Fi icon
                div {
                    style: "width: 1.75rem; height: 1.75rem; border: 1px solid rgba(255,255,255,0.25); border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: default; opacity: 0.7;",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "14",
                        height: "14",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "white",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M5 12.55a11 11 0 0 1 14.08 0" }
                        path { d: "M1.42 9a16 16 0 0 1 21.16 0" }
                        path { d: "M8.53 16.11a6 6 0 0 1 6.95 0" }
                        line { x1: "12", y1: "20", x2: "12.01", y2: "20" }
                    }
                }

                // Language
                div {
                    style: "width: 1.75rem; height: 1.75rem; border: 1px solid rgba(255,255,255,0.25); border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: default; opacity: 0.7; font-size: 11px; color: white; font-weight: 500;",
                    "EN"
                }
            }
        }
    }
}

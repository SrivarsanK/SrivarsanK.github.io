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
    // 12-hour format: "12:19 am"
    let time_str = time_val.format("%-I:%M %P").to_string();
    // "Thursday, 21 May"
    let date_str = time_val.format("%A, %-d %B").to_string();

    let bg_style = match &props.wallpaper {
        Some(w) => format!(
            "background-image: url({}); background-size: cover; background-position: center;",
            w
        ),
        None => "background-color: #1a1a2e;".to_string(),
    };

    rsx! {
        div {
            style: "position: fixed; inset: 0; z-index: 40; overflow: hidden; {bg_style}",

            // Very subtle backdrop blur to slightly soften the wallpaper
            div {
                style: "position: absolute; inset: 0; background: rgba(0,0,0,0.18); backdrop-filter: blur(3px); -webkit-backdrop-filter: blur(3px);"
            }

            // ── CLOCK — large, bottom-left ──
            div {
                style: "position: absolute; bottom: 2.5rem; left: 2.5rem; color: #fff; z-index: 10;",
                div {
                    style: "font-size: 4.5rem; font-weight: 300; font-family: 'JetBrains Mono', monospace; line-height: 1; letter-spacing: -1px; text-shadow: 0 2px 20px rgba(0,0,0,0.5);",
                    "{time_str}"
                }
                div {
                    style: "font-size: 1rem; font-weight: 400; opacity: 0.85; margin-top: 0.35rem; font-family: 'Inter', sans-serif; text-shadow: 0 1px 8px rgba(0,0,0,0.4);",
                    "{date_str}"
                }
            }

            // ── WIFI + LANGUAGE — bottom-right ──
            div {
                style: "position: absolute; bottom: 1.75rem; right: 1.75rem; display: flex; align-items: center; gap: 0.75rem; z-index: 10; color: rgba(255,255,255,0.75);",

                // WiFi icon (SVG)
                div {
                    style: "display: flex; align-items: center; gap: 0.25rem; font-size: 0.65rem; cursor: pointer; opacity: 0.8; transition: opacity 0.2s;",
                    title: "WiFi",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "18",
                        height: "18",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "white",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M5 12.55a11 11 0 0 1 14.08 0" }
                        path { d: "M1.42 9a16 16 0 0 1 21.16 0" }
                        path { d: "M8.53 16.11a6 6 0 0 1 6.95 0" }
                        circle { cx: "12", cy: "20", r: "1", fill: "white" }
                    }
                }

                // Language / keyboard
                div {
                    style: "font-size: 0.7rem; font-family: monospace; color: rgba(255,255,255,0.75); cursor: pointer; letter-spacing: 0.05em;",
                    "EN"
                }
            }

            // ── LOGIN CARD — centered ──
            div {
                style: "position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; z-index: 10;",

                // Avatar circle
                div {
                    style: "width: 6rem; height: 6rem; border-radius: 50%; background: linear-gradient(135deg, #6e6aca 0%, #8b5cf6 50%, #a78bfa 100%); display: flex; align-items: center; justify-content: center; margin-bottom: 1rem; box-shadow: 0 8px 32px rgba(0,0,0,0.4), inset 0 1px 0 rgba(255,255,255,0.2); border: 2px solid rgba(255,255,255,0.15);",
                    span {
                        style: "font-size: 2.5rem; font-weight: 700; color: #fff; font-family: 'Inter', sans-serif; text-shadow: 0 2px 8px rgba(0,0,0,0.3);",
                        "R"
                    }
                }

                // Username
                h1 {
                    style: "font-size: 1.4rem; font-weight: 600; color: #fff; margin: 0 0 0.75rem 0; text-shadow: 0 2px 12px rgba(0,0,0,0.5); font-family: 'Inter', sans-serif;",
                    "Ren"
                }

                // Login button / spinner
                if is_logging_in() {
                    div {
                        style: "display: flex; flex-direction: column; align-items: center; gap: 0.75rem; margin-top: 0.5rem;",
                        div {
                            style: "width: 1.5rem; height: 1.5rem; border: 3px solid rgba(255,255,255,0.25); border-top-color: #fff; border-radius: 50%; animation: spin 0.8s linear infinite;"
                        }
                        span {
                            style: "color: rgba(255,255,255,0.7); font-size: 0.8rem; font-family: monospace; animation: pulse 2s infinite;",
                            "Logging in..."
                        }
                    }
                } else {
                    button {
                        onclick: handle_login,
                        style: "display: flex; align-items: center; gap: 0.4rem; padding: 0.35rem 1.1rem; font-size: 0.78rem; font-family: monospace; color: rgba(255,255,255,0.9); background: rgba(255,255,255,0.12); border: 1px solid rgba(255,255,255,0.2); border-radius: 0.3rem; cursor: pointer; backdrop-filter: blur(8px); -webkit-backdrop-filter: blur(8px); transition: background 0.2s, transform 0.1s; letter-spacing: 0.03em;",
                        "Login →"
                    }
                }
            }
        }
    }
}

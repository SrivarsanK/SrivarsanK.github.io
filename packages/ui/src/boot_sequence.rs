use dioxus::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;

const BOOT_TEXT: &[&str] = &[
    "Ren BIOS v1.0.4 (C) 2024-2026",
    "CPU: Intel(R) Coffee Powered @ 4.20GHz",
    "Memory Test: 32768MB OK",
    "",
    "Checking Storage... DONE",
    "Checking Network... CONNECTED",
    "Loading Kernel... OK",
    "Initializing Global Modules... DONE",
    "Searching for User Profiles... FOUND",
    "",
    "Starting RenOS v2.1.0...",
];

#[component]
pub fn BootSequence(on_complete: EventHandler<()>) -> Element {
    let mut visible_lines = use_signal(|| 0_usize);

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        // Simple loop to reveal lines
        for _ in 0..BOOT_TEXT.len() {
            sleep(Duration::from_millis(100)).await;
            visible_lines.with_mut(|v| *v += 1);
        }
        
        // Wait a bit before completing
        sleep(Duration::from_millis(500)).await;
        on_complete.call(());
    });

    rsx! {
        div {
            class: "fixed inset-0 bg-black text-white font-mono p-4 sm:p-8 z-50 flex flex-col items-start overflow-hidden",
            div {
                class: "text-white mb-4 sm:mb-8 w-full overflow-hidden",
                pre {
                    class: "text-[8px] leading-[8px] sm:text-xs",
                    r#"
   ____                 ___  ____  
  |  _ \ ___ _ __      / _ \/ ___| 
  | |_) / _ \ '_ \    | | | \___ \ 
  |  _ <  __/ | | |   | |_| |___) |
  |_| \_\___|_| |_|    \___/|____/ 
"#
                }
            }
            div {
                class: "flex flex-col gap-1 w-full overflow-hidden",
                for i in 0..visible_lines() {
                    div {
                        key: "{i}",
                        class: "text-xs sm:text-sm typing-effect truncate",
                        "{BOOT_TEXT[i]}"
                    }
                }
            }
            div {
                class: "mt-4 sm:mt-8",
                div {
                    class: "flex items-center gap-2",
                    div {
                        class: "w-1.5 h-3 sm:w-2 sm:h-4 bg-white cursor-blink"
                    }
                }
            }
        }
    }
}

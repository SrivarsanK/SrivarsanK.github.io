use dioxus::prelude::*;
use ui::{BootSequence, LoginScreen};

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
                        // You can specify a wallpaper url here, or leave it None for black bg
                        wallpaper: None
                    }
                }
            }
            OsState::Desktop => {
                rsx! {
                    div {
                        class: "flex items-center justify-center h-screen w-screen bg-background",
                        div {
                            class: "text-terminal-green text-xl terminal-glow",
                            "Desktop Environment (Coming in Phase 4)"
                        }
                    }
                }
            }
        }
    }
}

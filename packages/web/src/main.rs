use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            class: "flex items-center justify-center h-screen w-screen bg-background",
            div {
                class: "text-terminal-green text-xl terminal-glow typing-effect",
                "OS Booting..."
            }
        }
    }
}

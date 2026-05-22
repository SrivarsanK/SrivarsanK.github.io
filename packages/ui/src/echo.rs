use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| {
                    response.set(event.value());
                },
            }

            if !response().is_empty() {
                p { 
                    "Server echoed: " 
                    i { "{response}" }
                }
            }
        }
    }
}
use dioxus::prelude::*;
use ui::{Hero, Echo};

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}

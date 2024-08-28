use dioxus::prelude::*;

mod export;
mod import;

#[component]
pub fn preset() -> Element {
    rsx! {
        div {
            class: "segment",
            div {
                class: "left",
                "Preset",
                import::import {},
                export::export {}
            }
        }
    }
}

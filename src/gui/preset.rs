use dioxus::prelude::*;

mod export;
mod import;

pub fn preset(cx: Scope) -> Element {
    render! {
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

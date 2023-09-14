use dioxus::prelude::*;

mod import;

pub fn preset(cx: Scope) -> Element {
    render! {
        div {
            class: "segment",
            div {
                class: "left",
                "Preset",
                import::import {}
            }
        }
    }
}

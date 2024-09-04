use dioxus::prelude::*;

mod export;
mod history;
mod import;

#[component]
pub fn preset() -> Element {
    rsx! {
        div {
            class: "segment",
            div {
                class: "center",
                "Preset",
                import::import {},
                export::export {},
                history::history {}
            }
        }
    }
}

use dioxus::prelude::*;

mod export;
pub mod history;
mod import;

#[component]
pub fn preset() -> Element {
    rsx! {
        div {
            class: "flex flex-wrap items-center gap-2",
            import::import {},
            export::export {},
            history::history {}
        }
    }
}

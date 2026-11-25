use crate::{
    gui::{help, theme},
    json::Preset,
};
use dioxus::prelude::*;

mod export;
pub mod history;
mod import;

#[component]
pub fn preset() -> Element {
    let mut state = use_context::<Signal<Preset>>();

    rsx! {
        div {
            class: "flex flex-wrap items-center gap-2",
            div {
                class: "relative inline-flex items-center gap-2",
                button {
                    r#type: "button",
                    id: "default-preset",
                    class: theme::BTN_SECONDARY,
                    onclick: move |_| {
                        let mut preset: Preset = serde_json::from_str("{}").unwrap();
                        preset.randomizer.seed = state().randomizer.seed;
                        state.set(preset);
                    },
                    "Default"
                }
                help::help { text: "Reset all settings to their default values" }
            }
            import::import {},
            export::export {},
            history::history {}
        }
    }
}

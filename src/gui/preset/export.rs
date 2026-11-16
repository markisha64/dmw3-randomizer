use std::fs::File;
use std::io::Write;

use crate::gui::{help, theme};
use crate::json::Preset;
use dioxus::prelude::*;

#[component]
pub fn export() -> Element {
    let state_signal = use_context::<Signal<Preset>>();
    let state = state_signal();

    rsx! {
        div {
            class: "relative inline-flex items-center gap-2",
            button {
                r#type: "button",
                id: "export",
                class: theme::BTN_SECONDARY,
                onclick: move |_| {
                    let json_str = serde_json::to_string::<Preset>(&state).unwrap();

                    let mut file = File::create("preset.json").unwrap();
                    let _ = file.write(json_str.as_bytes());
                },
                "Export"
            }
            help::help { text: "Exports to preset.json" }
        }
    }
}

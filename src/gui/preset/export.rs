use std::fs::File;
use std::io::Write;

use crate::json::Preset;
use dioxus::prelude::*;

#[component]
pub fn export() -> Element {
    let state_signal = use_context::<Signal<Preset>>();
    let state = state_signal();

    rsx! {
        div {
            class: "center tooltip",
            span {
                class: "tooltiptext",
                "Exports to preset.json"
            },
            label {
                r#for: "export",
                class: "file-upload",
                "Export"
            },
            input {
                r#type: "button",
                id: "export",
                onclick: move |_| {
                    let json_str = serde_json::to_string::<Preset>(&state).unwrap();

                    let mut file = File::create("preset.json").unwrap();
                    let _ = file.write(json_str.as_bytes());
                }
            }
        }
    }
}

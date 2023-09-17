use std::fs::File;
use std::io::Write;

use crate::json::Preset;
use dioxus::prelude::*;

use serde_json;

pub fn export(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();

    render! {
        div {
            class: "tooltip",
            span {
                class: "tooltiptext",
                "Exports to preset.json"
            },
            label {
                r#for: "export",
                class: "randomize",
                "Export"
            },
            input {
                r#type: "button",
                id: "export",
                onclick: move |_| {
                    let json_str = serde_json::to_string::<Preset>(&state.read()).unwrap();

                    let mut file = File::create("preset.json").unwrap();
                    let _ = file.write(json_str.as_bytes());
                }
            }
        }
    }
}

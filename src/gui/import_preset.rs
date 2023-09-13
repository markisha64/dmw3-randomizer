use crate::{gui::file_upload, json::Preset};
use dioxus::prelude::*;

use serde_json;
use std::fs;

pub fn import_preset(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();

    render! {
        div {
            class: "segment",
            "Preset",
            file_upload::file_upload {
                id: "import-preset",
                label: "Import",
                accept: ".json",
                onchange: move |x: Event<FormData>| {
                    if let Some(file_engine) = &x.files {
                        let files = file_engine.files();

                        match files.first() {
                            Some(file) => {
                                let json_str = fs::read_to_string(file).unwrap();

                                let json: Preset = serde_json::from_str(json_str.as_str()).unwrap();

                                (*state.write()) = json;
                            },
                            None => {}
                        }
                    }
                }
            }
        }
    }
}

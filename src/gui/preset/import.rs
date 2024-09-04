use crate::{gui::file_upload, json::Preset};

use dioxus::prelude::*;

use std::fs;

#[component]
pub fn import() -> Element {
    let mut state = use_context::<Signal<Preset>>();

    rsx! {
        file_upload::file_upload {
            id: "import-preset",
            label: "Import",
            accept: ".json",
            tooltip: "Import preset",
            onchange: move |x: Event<FormData>| {
                if let Some(file_engine) = &x.files() {
                    let files = file_engine.files();

                    match files.first() {
                        Some(file) => {
                            let json_str = fs::read_to_string(file).unwrap();

                            let json: Preset = serde_json::from_str(json_str.as_str()).unwrap();

                            state.set(json);
                        },
                        None => {}
                    }
                }
            }
        }
    }
}

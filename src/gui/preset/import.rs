use crate::{
    gui::{file_upload, GlobalState},
    json::Preset,
};

use dioxus::prelude::*;

use std::fs;

#[component]
pub fn import() -> Element {
    let mut state = use_context::<Signal<Preset>>();
    let mut global_state = use_context::<Signal<GlobalState>>();

    rsx! {
        file_upload::file_upload {
            id: "import-preset",
            label: "Import",
            accept: ".json",
            onchange: move |x: Event<FormData>| {
                if let Some(file_engine) = &x.files() {
                    let files = file_engine.files();

                    match files.first() {
                        Some(file) => {
                            let json_str = fs::read_to_string(file).unwrap();

                            let json: Preset = serde_json::from_str(json_str.as_str()).unwrap();

                            global_state.write().shop_limit_enabled = json.randomizer.shops.limit_shop_items.is_some();

                            state.set(json);
                        },
                        None => {}
                    }
                }
            }
        }
    }
}

use crate::{
    gui::{file_upload, GlobalState},
    json::Preset,
};

use dioxus::prelude::*;

use serde_json;
use std::fs;

pub fn import(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    render! {
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

                            (*global_state.write()).shop_limit_enabled = match json.randomizer.shops.limit_shop_items {
                                Some(_) => true,
                                None => false
                            };

                            (*state.write()) = json;
                        },
                        None => {}
                    }
                }
            }
        }
    }
}

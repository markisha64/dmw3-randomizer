use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use crate::json::Preset;

use crate::cli::Arguments;
use std::path::PathBuf;

mod checkbox;
mod encounters;
mod file_upload;
mod number_field;
mod parties;
mod preset;
mod randomize;
mod shops;
mod slider;

pub fn launch() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new().with_resizable(false)),
    );
}

pub struct GlobalState {
    pub shop_limit_enabled: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            shop_limit_enabled: true,
        }
    }
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider::<Arguments>(cx, || Arguments::default());
    use_shared_state_provider::<Preset>(cx, || serde_json::from_str("{}").unwrap());
    use_shared_state_provider::<GlobalState>(cx, || GlobalState::default());

    let state = use_shared_state::<Arguments>(cx).unwrap();
    let preset_state = use_shared_state::<Preset>(cx).unwrap();

    let read_state = state.read();

    let file_name = (*read_state).path.clone();

    let file_name_cl = match &file_name {
        Some(file) => file.file_name().unwrap().to_str().unwrap(),
        None => "Rom file",
    };

    let shuffles = preset_state.read().randomizer.shuffles;

    let seed = match read_state.seed {
        Some(s) => s,
        None => 64,
    };

    cx.render(rsx! {
        style { include_str!("../assets/style.css") },
        div {
            div {
                class: "inline",
                div {
                    class: "center",
                    file_upload::file_upload {
                        id: "rom-file",
                        label: "{file_name_cl}",
                        accept: ".bin",
                        onchange: move |x: Event<FormData>| {
                            if let Some(file_engine) = &x.files {
                                let files = file_engine.files();

                                match files.first() {
                                    Some(file) => (*state.write()).path = Some(PathBuf::from(file)),
                                    None => {}
                                }
                            }
                        }
                    },
                    number_field::number_field {
                        id: "shuffles",
                        value: shuffles as i64,
                        label: "Shuffles",
                        min: 1,
                        max: 255,
                        onchange: move |x: Event<FormData>| {
                            let shuffles = match x.data.value.parse::<u8>() {
                                Ok(sf) => {
                                    if 1 <= sf {
                                        sf
                                    } else {
                                       shuffles
                                    }
                                },
                                _ => shuffles
                            };

                            preset_state.write().randomizer.shuffles = shuffles;
                        }
                    },
                    div {
                        label {
                            r#for: "seed",
                            "Seed"
                        },
                        input {
                            style: "width: 120px;",
                            r#type: "number",
                            id: "seed",
                            value: "{seed}",
                            onchange: move |x| {
                                if x.data.value == "" {
                                    state.write().seed = Some(64);
                                } else {
                                    state.write().seed = Some(x.data.value.parse::<u64>().unwrap());
                                }
                            }
                        },
                    },
                    randomize::randomize {}
                },
                preset::preset {}
            },
        },
        encounters::encounters {},
        parties::parties {}
        shops::shops {}
    })
}

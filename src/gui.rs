use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;

use crate::json::Preset;

use crate::cli::Arguments;
use rand_xoshiro::Xoshiro256StarStar;
use std::path::PathBuf;

mod checkbox;
mod encounters;
mod file_upload;
mod maps;
mod number_field;
mod number_field_float;
mod parties;
mod preset;
mod randomize;
mod scaling;
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

    let rng_state = use_state::<Xoshiro256StarStar>(cx, || {
        Xoshiro256StarStar::seed_from_u64(state.read().seed.unwrap())
    });

    let mut rng = rng_state.get().clone();

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

    let output_file = match &(*read_state).output {
        Some(x) => x.clone(),
        None => String::from(format!("{}", seed)),
    };

    cx.render(rsx! {
        div {
            link { href: "../assets/style.css", rel: "stylesheet" },
            link { href: "style.css", rel: "stylesheet" },
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
                                let seed = match x.data.value.parse::<u64>() {
                                    Ok(sd) => sd,
                                    _ => seed
                                };

                                state.write().seed = Some(seed);
                            }
                        },
                        label {
                            r#for: "new",
                            class: "randomize",
                            "New",
                        },
                        input {
                            r#type: "button",
                            id: "new",
                            onclick: move |_| {
                                state.write().seed = Some(rng.next_u64());
                                rng_state.modify(|_| rng.clone());
                            }
                        }
                    },
                    randomize::randomize {}
                },
                div {
                    class: "center",
                    preset::preset {},
                    div {
                        label {
                            r#for: "test",
                            "File name"
                        }
                        input {
                            id: "test",
                            r#type: "text",
                            value: "{output_file}",
                            minlength: 1,
                            maxlength: 20,
                            onchange: move |x| {
                                if x.value == "" {
                                    return;
                                }

                                state.write().output = Some(x.value.clone());
                            }
                        }
                    },
                },
            },
        },
        encounters::encounters {},
        scaling::scaling {},
        parties::parties {},
        shops::shops {},
        maps::maps {}
    })
}

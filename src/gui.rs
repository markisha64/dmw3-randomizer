use chrono::Utc;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;

use crate::gui::preset::history::get_mapped;
use crate::json::Preset;

use crate::cli::Arguments;
use crate::db;
use rand_xoshiro::Xoshiro256StarStar;
use std::path::PathBuf;

mod checkbox;
mod encounters;
mod file_upload;
mod maps;
mod models;
mod number_field;
mod number_field_float;
mod parties;
mod preset;
mod randomize;
mod scaling;
mod shops;
mod slider;

pub fn launch_app() {
    LaunchBuilder::desktop()
        .with_cfg(Config::default().with_window(WindowBuilder::new().with_resizable(false)))
        .launch(app);
}

fn app() -> Element {
    use_context_provider(|| Signal::new(get_mapped()));
    use_context_provider(|| {
        let last = db::last();

        let mut args = match last {
            Ok(history) => {
                serde_json::from_str(history.arguments.as_str()).unwrap_or(Arguments::default())
            }
            Err(_) => Arguments::default(),
        };

        args.path = match args.path {
            Some(path) => match path.exists() {
                true => Some(path),
                false => None,
            },
            None => None,
        };

        args.seed = Some(Utc::now().timestamp() as u64);

        Signal::new(args)
    });
    use_context_provider(|| {
        Signal::new(match db::last() {
            Ok(history) => serde_json::from_str::<Preset>(history.preset.as_str())
                .unwrap_or(serde_json::from_str("{}").unwrap()),
            Err(_) => serde_json::from_str("{}").unwrap(),
        })
    });

    let mut state = use_context::<Signal<Arguments>>();
    let mut preset_state = use_context::<Signal<Preset>>();

    let mut rng_state = use_signal::<Xoshiro256StarStar>(|| {
        Xoshiro256StarStar::seed_from_u64(state.read().seed.unwrap())
    });

    let read_state = state.read();

    let file_name = read_state.path.clone();

    let file_name_cl: String = match &file_name {
        Some(file) => String::from(file.file_name().unwrap().to_str().unwrap()),
        None => String::from("Rom file"),
    };

    let shuffles = preset_state.read().randomizer.shuffles;

    let seed = read_state.seed.unwrap_or(64);

    let output_file = match &read_state.output {
        Some(x) => x.clone(),
        None => format!("{}", seed),
    };

    rsx! {
        div {
            link { href: "../assets/style.css", rel: "stylesheet" },
            link { href: "style.css", rel: "stylesheet" },
            div {
                class: "inline",
                div {
                    class: "center",
                    file_upload::file_upload {
                        id: "rom-file",
                        label: file_name_cl,
                        accept: ".bin",
                        onchange: move |x: Event<FormData>| {
                            if let Some(file_engine) = x.files() {
                                let files = file_engine.files();

                                match files.first() {
                                    Some(file) => state.write().path = Some(PathBuf::from(file)),
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
                        onchange: move |x: i64| {
                            preset_state.write().randomizer.shuffles = x as u8;
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
                                let seed = match x.data.value().parse::<u64>() {
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
                                state.write().seed = Some(rng_state.write().next_u64());
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
                                if x.value() == "" {
                                    return;
                                }

                                state.write().output = Some(x.value().clone());
                            }
                        }
                    },
                },
            },
        },
        div {
            class: "left",
            div {
                class: "column",
                parties::parties {},
                scaling::scaling {},
            },
            div {
                class: "column",
                shops::shops {},
                encounters::encounters {},
                maps::maps {},
                models::models {}
            },
        }
    }
}

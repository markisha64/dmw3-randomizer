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
mod party_exp_bits;
mod preset;
mod randomize;
mod scaling;
mod shops;
mod slider;

pub fn launch_app() {
    LaunchBuilder::desktop()
        .with_cfg(Config::default().with_window(WindowBuilder::new().with_resizable(true)))
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

    let mut state_signal = use_context::<Signal<Arguments>>();
    let mut preset_state = use_context::<Signal<Preset>>();

    let state = state_signal();

    let mut rng_state =
        use_signal::<Xoshiro256StarStar>(|| Xoshiro256StarStar::seed_from_u64(state.seed.unwrap()));

    let file_name = state.path.clone();

    let file_name_cl: String = match &file_name {
        Some(file) => String::from(file.file_name().unwrap().to_str().unwrap()),
        None => String::from("Rom file"),
    };

    let seed = state.seed.unwrap_or(64);

    let output_file = match &state.output {
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
                                    Some(file) => state_signal.write().path = Some(PathBuf::from(file)),
                                    None => {}
                                }
                            }
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

                                state_signal.write().seed = Some(seed);
                                preset_state.write().randomizer.seed = seed;
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
                                state_signal.write().seed = Some(rng_state.write().next_u64());
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

                                state_signal.write().output = Some(x.value().clone());
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
                models::models {},
            },
            div {
                class: "column",
                shops::shops {},
                encounters::encounters {},
                maps::maps {},
                party_exp_bits::party_exp_bits {},
            },
        }
    }
}

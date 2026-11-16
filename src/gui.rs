use chrono::Utc;
use dioxus::desktop::{Config, LogicalSize, WindowBuilder};
use dioxus::prelude::*;
use rand_xoshiro::rand_core::RngCore;
use rand_xoshiro::rand_core::SeedableRng;

use crate::json::Preset;

use crate::cli::Arguments;
use crate::db;
use rand_xoshiro::Xoshiro256StarStar;
use std::path::PathBuf;

mod auctions;
mod card_game;
mod checkbox;
mod drawer;
mod encounters;
mod file_upload;
mod help;
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
mod theme;

pub fn launch_app() {
    LaunchBuilder::desktop()
        .with_cfg(Config::default().with_window(
            WindowBuilder::new()
                .with_title("Digimon World 3/2003 Randomizer")
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(1200.0, 850.0)),
        ))
        .launch(app);
}

fn app() -> Element {
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
        link { href: "tailwind.css", rel: "stylesheet" },
        link { href: "../assets/tailwind.css", rel: "stylesheet" },
        div {
            class: "mx-auto flex min-h-screen max-w-[1600px] flex-col gap-6 p-4 sm:p-6 lg:p-8",
            header {
                class: "rounded-2xl border border-white/10 bg-dmw/80 p-4 shadow-2xl shadow-black/30 backdrop-blur-sm",
                div {
                    class: "flex flex-wrap items-center gap-4",
                    div {
                        class: "mr-auto min-w-0",
                        h1 {
                            class: "truncate text-lg font-bold tracking-wide text-white",
                            "Digimon World 3/2003 Randomizer"
                        }
                    }
                    div {
                        class: "flex flex-wrap items-center gap-2",
                        file_upload::file_upload {
                            id: "rom-file",
                            label: format!("ROM: {file_name_cl}"),
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
                        }
                        div {
                            class: "flex items-center gap-2 rounded-lg border border-white/15 bg-dmw px-3 py-1.5 transition focus-within:border-sky-300/60",
                            label {
                                r#for: "seed",
                                class: "text-[11px] font-semibold uppercase tracking-wider text-white/40",
                                "Seed"
                            }
                            input {
                                id: "seed",
                                r#type: "number",
                                value: "{seed}",
                                class: "w-28 bg-transparent text-sm text-white outline-none",
                                onchange: move |x| {
                                    let seed = match x.data.value().parse::<u64>() {
                                        Ok(sd) => sd,
                                        _ => seed
                                    };

                                    state_signal.write().seed = Some(seed);
                                    preset_state.write().randomizer.seed = seed;
                                }
                            }
                            button {
                                r#type: "button",
                                class: theme::BTN_SECONDARY,
                                onclick: move |_| {
                                    state_signal.write().seed = Some(rng_state.write().next_u64());
                                },
                                "New"
                            }
                        }
                        div {
                            class: "flex items-center gap-2 rounded-lg border border-white/15 bg-dmw px-3 py-1.5 transition focus-within:border-sky-300/60",
                            label {
                                r#for: "test",
                                class: "text-[11px] font-semibold uppercase tracking-wider text-white/40",
                                "Output"
                            }
                            input {
                                id: "test",
                                r#type: "text",
                                value: "{output_file}",
                                minlength: 1,
                                maxlength: 20,
                                class: "w-32 bg-transparent text-sm text-white outline-none",
                                onchange: move |x| {
                                    if x.value() == "" {
                                        return;
                                    }

                                    state_signal.write().output = Some(x.value().clone());
                                }
                            }
                        }
                        randomize::randomize {}
                    }
                }
            }
            main {
                class: "flex flex-col gap-4 md:flex-row md:items-start",
                div {
                    class: "flex w-full flex-col gap-4",
                    drawer::drawer {
                        title: "Parties",
                        subtitle: "Starting parties, stats and digivolutions",
                        default_open: true,
                        parties::parties {}
                    }
                    drawer::drawer {
                        title: "Scaling",
                        subtitle: "Enemy stat scaling",
                        scaling::scaling {}
                    }
                    drawer::drawer {
                        title: "Models",
                        subtitle: "Digimon and stage model colors",
                        models::models {}
                    }
                    drawer::drawer {
                        title: "Card Game",
                        subtitle: "Cards, boosters and prices",
                        card_game::card_game {}
                    }
                    drawer::drawer {
                        title: "Auctions",
                        subtitle: "Auction stock and values",
                        auctions::auctions {}
                    }
                }
                div {
                    class: "flex w-full flex-col gap-4",
                    drawer::drawer {
                        title: "Shops",
                        subtitle: "Shop stock and prices",
                        shops::shops {}
                    }
                    drawer::drawer {
                        title: "Encounters",
                        subtitle: "Wild encounters, bosses and TNT",
                        default_open: true,
                        encounters::encounters {}
                    }
                    drawer::drawer {
                        title: "Maps",
                        subtitle: "Colors, music, item boxes and Mobius Desert",
                        maps::maps {}
                    }
                    drawer::drawer {
                        title: "Party DV Exp / Exp / Bits",
                        subtitle: "Post-battle gains",
                        party_exp_bits::party_exp_bits {}
                    }
                    drawer::drawer {
                        title: "Preset",
                        subtitle: "Import, export and history",
                        preset::preset {}
                    }
                }
            }
        }
    }
}

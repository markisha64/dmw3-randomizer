use crate::dump::create_spoiler;
use crate::gui::theme;
use crate::{cli::Arguments, db, json::Preset, mkpsxiso, patch};

use anyhow::{anyhow, Context};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
enum Steps {
    #[default]
    Input,
    Extracting,
    Randomizing,
    Packaging,
}

impl Steps {
    fn randomizing(&self) -> bool {
        match self {
            Steps::Input => false,
            _ => true,
        }
    }

    fn to_percent(&self) -> u8 {
        match self {
            Steps::Input => 0,
            Steps::Extracting => 33,
            Steps::Randomizing => 66,
            Steps::Packaging => 100,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Steps::Input => "Randomize",
            Steps::Extracting => "Extracting ROM",
            Steps::Randomizing => "Randomizing",
            Steps::Packaging => "Packaging ROM",
        }
    }
}

#[component]
pub fn randomize() -> Element {
    let mut state = use_signal::<Steps>(Steps::default);
    let args_state = use_context::<Signal<Arguments>>();
    let preset_state = use_context::<Signal<Preset>>();

    let current = state();
    let percent = current.to_percent();
    let label = current.label();
    let is_randomizing = current.randomizing();

    let args = args_state();
    let preset = preset_state();

    rsx! {
        button {
            r#type: "button",
            class: theme::BTN_PRIMARY,
            disabled: is_randomizing,
            onclick: move |_| {
                to_owned!(preset, args);
                let current_state = state();

                if !current_state.randomizing() {
                    state.set(Steps::Extracting);

                    spawn(async move {
                        let r: anyhow::Result<()> = async move {
                            let path = &args.path.as_ref().context("missing path")?;

                            preset.randomizer.seed = match &args.seed {
                                Some(seed) => *seed,
                                None => preset.randomizer.seed,
                            };

                            db::insert(&preset, &args).map_err(|_| anyhow!("failed to insert to db"))?;

                            let file_name = args.output.unwrap_or(format!("{}", preset.randomizer.seed));

                            mkpsxiso::extract(path).await?;

                            state.set(Steps::Randomizing);

                            let rom_name = path
                                .file_name()
                                .context("Failed file name get")?
                                .to_str()
                                .context("Failed to_str conversion")?;

                            let objects = patch(path, &preset).await?;

                            create_spoiler(&objects, path, file_name.as_str()).await?;

                            state.set(Steps::Packaging);

                            mkpsxiso::build(rom_name, &file_name).await?;

                            state.set(Steps::Input);

                            Ok(())
                        }.await;

                        if let Err(err) = r {
                            println!("encountered err {}", err);
                        }

                        state.set(Steps::Input);
                    });
                }
            },
            if is_randomizing {
                div {
                    class: "absolute inset-x-0 bottom-0 h-1 bg-slate-950/20",
                    div {
                        class: "h-full bg-slate-950/80 transition-all duration-500",
                        style: "width:{percent}%;",
                    }
                }
                span {
                    class: "relative flex items-center gap-2",
                    "{label}"
                    span { class: "font-mono text-xs opacity-70", "{percent}%" }
                }
            } else {
                span { class: "relative", "Randomize" }
            }
        }
    }
}

use crate::dump::create_spoiler;
use crate::gui::preset::history::{get_mapped, HistoryMapped};
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
}

#[component]
pub fn randomize() -> Element {
    let mut state = use_signal::<Steps>(Steps::default);
    let args_state = use_context::<Signal<Arguments>>();
    let mut preset_state = use_context::<Signal<Preset>>();
    let mut history_state = use_context::<Signal<Vec<HistoryMapped>>>();

    let percent = state().to_percent();

    rsx! {
        label {
            r#for: "randomize",
            class: "randomize",
            if state().randomizing()  {
                div {
                    r#style: "height: 100%; width:{percent}%;",
                    div {
                        class: "progress"
                    }
                }
            } else {
                "Randomize"
            }
        },
        input {
            r#type: "button",
            id: "randomize",
            onclick: move |_| {
                let current_state = state();

                if !current_state.randomizing() {
                    state.set(Steps::Extracting);

                    let args = args_state();
                    let preset = preset_state();

                    spawn(async move {
                        let r: anyhow::Result<()> = async move {
                            let path = &args.path.as_ref().context("missing path")?;

                            preset_state.write().randomizer.seed = args.seed.unwrap_or(preset.randomizer.seed);

                            db::insert(&preset, &args).map_err(|_| anyhow!("failed to insert to db"))?;
                            history_state.set(get_mapped());

                            let file_name = args.output.unwrap_or(format!("{}", preset.randomizer.seed));

                            if !mkpsxiso::extract(path).await? {
                                panic!("Error extracting");
                            }

                            state.set(Steps::Randomizing);

                            let rom_name = path
                                .file_name()
                                .context("Failed file name get")?
                                .to_str()
                                .context("Failed to_str conversion")?;

                            let objects = patch(path, &preset).await?;

                            create_spoiler(&objects, path, file_name.as_str()).await?;

                            state.set(Steps::Packaging);

                            if !mkpsxiso::build(rom_name, &file_name).await? {
                                panic!("Error repacking")
                            }

                            state.set(Steps::Input);

                            Ok(())
                        }.await;

                        if let Err(err) = r {
                            println!("encountered err {}", err);
                        }

                        state.set(Steps::Input);
                    });
                }
            }
        },
    }
}

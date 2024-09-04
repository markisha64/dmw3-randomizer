use crate::gui::preset::history::{get_mapped, HistoryMapped};
use crate::{cli::Arguments, db, json::Preset, mkpsxiso, patch};

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

    let percent = state.read().to_percent();

    rsx! {
        label {
            r#for: "randomize",
            class: "randomize",
            if state.read().randomizing()  {
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
                let current_state = *state.read();

                if !current_state.randomizing() {
                    state.set(Steps::Extracting);

                    let args = args_state.read().clone();

                    spawn(async move {
                        match &args.path {
                            Some(path) => {
                                preset_state.write().randomizer.seed = match args.seed {
                                    Some(seed) => seed,
                                    None => preset_state.read().randomizer.seed,
                                };

                                let preset = preset_state.read().clone();

                                db::insert(&preset, &args).unwrap();
                                history_state.set(get_mapped());

                                let file_name = match &args.output {
                                    Some(name) => name.clone(),
                                    None => format!("{}", preset.randomizer.seed)
                                };

                                if !mkpsxiso::extract(path).await.unwrap() {
                                    panic!("Error extracting");
                                }

                                state.set(Steps::Randomizing);

                                patch(path, &preset).await;

                                state.set(Steps::Packaging);

                                if !mkpsxiso::build(&file_name).await.unwrap() {
                                    panic!("Error repacking")
                                }

                                state.set(Steps::Input);
                            },
                            None => {}
                        }

                        state.set(Steps::Input);
                    });
                }
            }
        },
    }
}

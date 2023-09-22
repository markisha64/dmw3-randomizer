use crate::{cli::Arguments, json::Preset, mkpsxiso, patch};

use dioxus::prelude::*;
use futures_util::stream::StreamExt;

#[derive(Clone, Copy, PartialEq)]
enum Steps {
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

impl Default for Steps {
    fn default() -> Self {
        Steps::Input
    }
}

pub fn randomize(cx: Scope) -> Element {
    let state = use_state::<Steps>(cx, || Steps::default());
    let args_state = use_shared_state::<Arguments>(cx).unwrap();
    let preset_state = use_shared_state::<Preset>(cx).unwrap();

    let percent = state.to_percent();

    let set_percent = use_coroutine(cx, |mut rx: UnboundedReceiver<Steps>| {
        to_owned![state];

        async move {
            while let Some(x) = rx.next().await {
                state.modify(|_| x);
            }
        }
    });

    render! {
        label {
            r#for: "randomize",
            class: "randomize",
            if state.randomizing()  {
                rsx! {
                    div {
                        r#style: "height: 100%; width:{percent}%;",
                        div {
                            class: "progress"
                        }
                    }
                }
            } else {
                rsx! {
                    "Randomize"
                }
            }
        },
        input {
            r#type: "button",
            id: "randomize",
            onclick: move |_| {
                let current_state = (*state.get()).clone();

                to_owned![args_state, preset_state, set_percent];

                let args = args_state.read().clone();
                let mut preset = preset_state.read().clone();

                if !current_state.randomizing() {
                    state.modify(|_| Steps::Extracting);

                    cx.spawn(async move{
                        let _ = tokio::spawn(async move {
                            match &args.path {
                                Some(path) => {
                                    preset.randomizer.seed = match &args.seed {
                                        Some(seed) => *seed,
                                        None => preset.randomizer.seed,
                                    };

                                    let file_name = match args.output {
                                        Some(name) => name,
                                        None => format!("{}", preset.randomizer.seed)
                                    };

                                    if !mkpsxiso::extract(&path) {
                                        panic!("Error extracting");
                                    }

                                    set_percent.send(Steps::Randomizing);

                                    patch(path, &preset);

                                    set_percent.send(Steps::Packaging);

                                    if !mkpsxiso::build(&file_name) {
                                        panic!("Error repacking")
                                    }

                                    set_percent.send(Steps::Input);
                                },
                                None => {}
                            }

                            set_percent.send(Steps::Input);
                        }).await;
                    });
                }
            }
        },
    }
}

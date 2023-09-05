use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use super::cli::Arguments;
use std::path::PathBuf;

mod checkbox;
mod randomize;

pub fn launch() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new().with_resizable(false)),
    );
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider::<Arguments>(cx, || Arguments::default());
    let state = use_shared_state::<Arguments>(cx).unwrap();

    let read_state = state.read();

    let file_name = (*read_state).path.clone();

    let file_name_cl = match &file_name {
        Some(file) => file.file_name().unwrap().to_str().unwrap(),
        None => "Rom file",
    };

    let seed = match read_state.seed {
        Some(s) => s,
        None => 64,
    };

    cx.render(rsx! {
        link {
            href: "src/gui/style.css",
            rel: "stylesheet"
        },
        div {
            class: "inline",
            div {
                class: "center",
                label {
                    r#for: "file-upload",
                    class: "file-upload",
                    "{file_name_cl}"
                },
                input {
                    r#type: "file",
                    accept: ".bin",
                    id: "file-upload",
                    multiple: false,
                    onchange: move |x| {
                        if let Some(file_engine) = &x.files {
                            let files = file_engine.files();

                            match files.first() {
                                Some(file) => (*state.write()).path = Some(PathBuf::from(file)),
                                None => {}
                            }
                        }
                    },
                },
                div {
                    label {
                        r#for: "seed",
                        "Seed"
                    },
                    input {
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
            }
        },
        div {
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Encounters",
                    id: "encounters.enabled",
                    onchange: |_| {}
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Cardmon",
                    id: "encounters.cardmon",
                    onchange: |_| {}
                },
                checkbox::checkbox {
                    label: "Bosses",
                    checked: true,
                    id: "encounters.bosses",
                    onchange: |_| {}
                },
            }
        }
    })
}

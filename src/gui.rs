use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use crate::json::Preset;

use crate::cli::Arguments;
use std::path::PathBuf;

mod checkbox;
mod encounters;
mod parties;
mod randomize;
mod shops;

pub fn launch() {
    dioxus_desktop::launch_cfg(
        app,
        Config::default().with_window(WindowBuilder::new().with_resizable(false)),
    );
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider::<Arguments>(cx, || Arguments::default());
    use_shared_state_provider::<Preset>(cx, || serde_json::from_str("{}").unwrap());

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
        encounters::encounters {},
        parties::parties {}
        shops::shops {}
    })
}

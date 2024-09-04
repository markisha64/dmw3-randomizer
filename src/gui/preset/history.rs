use crate::{cli::Arguments, db, json::Preset};

use chrono::DateTime;
use dioxus::prelude::*;

#[component]
pub fn history() -> Element {
    let mut preset_state = use_context::<Signal<Preset>>();
    let mut args_state = use_context::<Signal<Arguments>>();

    let history = db::history().ok()?;

    let history_mapped: Vec<_> = history
        .iter()
        .map(|history| {
            let preset: Preset = serde_json::from_str(&history.preset).unwrap();
            let timestamp = DateTime::from_timestamp(history.created_at, 0)
                .expect("invalid timestamt")
                .naive_local();

            (
                history.created_at,
                timestamp,
                preset.randomizer.seed,
                preset,
            )
        })
        .collect();

    rsx! {
        div {
            class: "tooltip",
            span {
                class: "tooltiptext",
                "Show previous randomizations"
            },
            label {
                r#for: "history",
                class: "file-upload",
                "History"
            },
            input {
                r#type: "button",
                id: "history",
                onclick: |_| {
                    eval("document.getElementById(\"history_dialog\").showModal();");
                }
            }
        },
        dialog {
            id: "history_dialog",
            div {
                class: "center",
                table {
                    tr {
                        th { "Timestamp" },
                        th { "Seed" },
                    }
                    for entry in history_mapped {
                        tr {
                            onclick: move |_| {
                                eval(format!("document.getElementById(\"history_dialog\").close();").as_str());
                                preset_state.set(entry.3.clone());
                                args_state.write().seed = Some(entry.2);
                            },
                            td {
                                "{entry.1}"
                            },
                            td {
                                "{entry.2}"
                            }
                        }
                    }
                }
            }
        }
    }
}

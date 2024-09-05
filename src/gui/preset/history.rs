use std::{fs::File, io::Write};

use crate::{
    cli::Arguments,
    db::{self},
    json::Preset,
};

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use dioxus::prelude::*;

pub type HistoryMapped = (i64, NaiveDateTime, u64, Preset, Preset);

pub fn get_mapped() -> Vec<HistoryMapped> {
    let history = db::history().unwrap_or(Vec::new());

    let history_mapped: Vec<_> = history
        .iter()
        .map(|history| {
            let preset: Preset = serde_json::from_str(&history.preset).unwrap();
            let timestamp = Local
                .from_utc_datetime(
                    &DateTime::from_timestamp(history.created_at, 0)
                        .expect("timestamp error")
                        .naive_utc(),
                )
                .naive_local();

            (
                history.created_at,
                timestamp,
                preset.randomizer.seed,
                preset.clone(),
                preset,
            )
        })
        .collect();

    history_mapped
}

#[component]
pub fn history() -> Element {
    let mut preset_state = use_context::<Signal<Preset>>();
    let mut args_state = use_context::<Signal<Arguments>>();
    let state = use_context::<Signal<Vec<HistoryMapped>>>();

    let history = state.read().clone();

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
                        th {
                            colspan: 2,
                            ""
                        }
                    }
                    for entry in history {
                        tr {
                            td {
                                "{entry.1}"
                            },
                            td {
                                "{entry.2}"
                            },
                            td {
                                div {
                                    class: "center",
                                    label {
                                        r#for: "apply_{entry.0}",
                                        class: "file-upload",
                                        "Apply"
                                    },
                                    input {
                                        r#type: "button",
                                        id: "apply_{entry.0}",
                                        onclick: move |_| {
                                            eval(format!("document.getElementById(\"history_dialog\").close();").as_str());
                                            preset_state.set(entry.3.clone());
                                            args_state.write().seed = Some(entry.2);
                                        },
                                    }
                                }
                            },
                            td {
                                div {
                                    class: "center",
                                    label {
                                        r#for: "export_{entry.0}",
                                        class: "file-upload",
                                        "Export"
                                    },
                                    input {
                                        r#type: "button",
                                        id: "export_{entry.0}",
                                        onclick: move |_| {
                                            eval(format!("document.getElementById(\"history_dialog\").close();").as_str());
                                            let json_str = serde_json::to_string::<Preset>(&entry.4).unwrap();

                                            let mut file = File::create("preset.json").unwrap();
                                            let _ = file.write(json_str.as_bytes());
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

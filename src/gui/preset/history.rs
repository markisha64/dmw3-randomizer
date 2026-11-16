use std::{fs::File, io::Write};

use crate::{
    cli::Arguments,
    db::{self},
    gui::{help, theme},
    json::Preset,
};

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use dioxus::prelude::*;

const INITIAL_HISTORY_LIMIT: usize = 100;

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
    let mut history = use_signal::<Vec<HistoryMapped>>(Vec::new);
    let mut show = use_signal(|| false);
    let mut show_all = use_signal(|| false);

    let (displayed, total) = if show() {
        let entries = history.read();
        let total = entries.len();
        let take = if show_all() {
            total
        } else {
            INITIAL_HISTORY_LIMIT.min(total)
        };

        (
            Some(entries.iter().take(take).cloned().collect::<Vec<_>>()),
            total,
        )
    } else {
        (None, 0)
    };

    let shown = displayed.as_ref().map(|entries| entries.len()).unwrap_or(0);

    rsx! {
        div {
            class: "relative inline-flex items-center gap-2",
            button {
                r#type: "button",
                id: "history",
                class: theme::BTN_SECONDARY,
                onclick: move |_| {
                    show_all.set(false);
                    history.set(get_mapped());
                    show.set(true);
                    eval("document.getElementById(\"history_dialog\").showModal();");
                },
                "History"
            }
            help::help { text: "Show previous randomizations" }
        }
        dialog {
            id: "history_dialog",
            class: "fixed inset-0 m-auto w-[min(92vw,48rem)] max-h-[85vh] overflow-hidden rounded-2xl border border-white/15 bg-dmw-panel p-0 text-white shadow-2xl",
            div {
                class: "flex max-h-[85vh] flex-col",
                div {
                    class: "flex items-center justify-between border-b border-white/10 px-5 py-4",
                    div {
                        h2 { class: "text-lg font-semibold text-white", "History" }
                        p {
                            class: "text-xs text-white/40",
                            if shown < total {
                                "Showing latest {shown} of {total}"
                            } else {
                                "Previous randomizations"
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "rounded-md p-1.5 text-white/50 transition hover:bg-white/10 hover:text-white",
                        onclick: move |_| {
                            show.set(false);
                            show_all.set(false);
                            eval("document.getElementById(\"history_dialog\").close();");
                        },
                        "✕"
                    }
                }
                if let Some(displayed) = displayed {
                    if displayed.is_empty() {
                        div {
                            class: "px-5 py-10 text-center text-sm text-white/40",
                            "No randomizations yet."
                        }
                    } else {
                        div {
                            class: "min-h-0 overflow-auto",
                            table {
                                class: "w-full border-collapse text-sm",
                                thead {
                                    class: "sticky top-0 z-10 bg-dmw-panel text-left text-[11px] uppercase tracking-wider text-white/40",
                                    tr {
                                        th { class: "px-4 py-2 font-semibold", "Timestamp" }
                                        th { class: "px-4 py-2 font-semibold", "Seed" }
                                        th { class: "px-4 py-2 font-semibold", "" }
                                        th { class: "px-4 py-2 font-semibold", "" }
                                    }
                                }
                                tbody {
                                    for entry in displayed {
                                        tr {
                                            class: "border-t border-white/5 transition hover:bg-white/5",
                                            td { class: "px-4 py-2 text-white/70", "{entry.1}" }
                                            td { class: "px-4 py-2 font-mono text-white/80", "{entry.2}" }
                                            td {
                                                class: "px-4 py-2",
                                                div {
                                                    class: "flex justify-end",
                                                    button {
                                                        r#type: "button",
                                                        id: "apply_{entry.0}",
                                                        class: theme::BTN_SECONDARY,
                                                        onclick: move |_| {
                                                            show.set(false);
                                                            show_all.set(false);
                                                            eval(format!("document.getElementById(\"history_dialog\").close();").as_str());
                                                            preset_state.set(entry.3.clone());
                                                            args_state.write().seed = Some(entry.2);
                                                        },
                                                        "Apply"
                                                    }
                                                }
                                            }
                                            td {
                                                class: "px-4 py-2",
                                                div {
                                                    class: "flex justify-end",
                                                    button {
                                                        r#type: "button",
                                                        id: "export_{entry.0}",
                                                        class: theme::BTN_SECONDARY,
                                                        onclick: move |_| {
                                                            show.set(false);
                                                            show_all.set(false);
                                                            eval(format!("document.getElementById(\"history_dialog\").close();").as_str());
                                                            let json_str = serde_json::to_string::<Preset>(&entry.4).unwrap();

                                                            let mut file = File::create("preset.json").unwrap();
                                                            let _ = file.write(json_str.as_bytes());
                                                        },
                                                        "Export"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if shown < total {
                            div {
                                class: "flex items-center justify-between gap-3 border-t border-white/10 px-4 py-3",
                                span {
                                    class: "text-xs text-white/40",
                                    "Showing latest {shown} of {total}"
                                }
                                button {
                                    r#type: "button",
                                    class: theme::BTN_SECONDARY,
                                    onclick: move |_| show_all.set(true),
                                    "Show all"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

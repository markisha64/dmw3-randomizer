use dioxus::prelude::*;

use crate::json::Preset;

use crate::gui::{checkbox, number_field};

pub fn parties(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.parties.enabled;
    let random_parties = read_state.randomizer.parties.parties;

    let stat_distribution = read_state.randomizer.parties.stat_distribution;
    let min_stat = read_state.randomizer.parties.min_starting_stat;
    let total_start_stat = read_state.randomizer.parties.total_starting_stats;

    let res_distribution = read_state.randomizer.parties.res_distribution;
    let min_res = read_state.randomizer.parties.min_starting_res;
    let total_start_res = read_state.randomizer.parties.total_starting_res;

    let stat_affinities = read_state.randomizer.parties.stat_affinities;
    let res_affinities = read_state.randomizer.parties.res_affinities;

    let learned_tech = read_state.randomizer.parties.learned_tech;
    let signatures = read_state.randomizer.parties.signatues;

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Party",
                    id: "party.enabled",
                    checked: enabled,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.enabled = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Parties",
                    id: "parties.random_parties",
                    checked: random_parties,
                    disabled: !enabled,
                    tooltip: "Randomize parties (preview currently unavailable)",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.parties = x.data.value == "true";
                    }
                },
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Stat distribution",
                    id: "parties.stat_distribution",
                    checked: stat_distribution,
                    disabled: !enabled,
                    tooltip: "Randomize player digimon stat distribution",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.stat_distribution = x.data.value == "true";
                    }
                },
                number_field::number_field {
                    label: "Min stat",
                    id: "parties.min_stat",
                    value: min_stat as i64,
                    disabled: !stat_distribution || !enabled,
                    min: 1,
                    max: (total_start_stat / 5) as i64,
                    tooltip: "Mininum points per stat",
                    onchange: move |x: Event<FormData>| {
                        let new_stat = match x.data.value.parse::<u16>() {
                            Ok(s) => {
                                if s * 5 > total_start_stat {
                                    min_stat
                                } else {
                                    s
                                }
                            },
                            _ => min_stat
                        };

                        state.write().randomizer.parties.min_starting_stat = new_stat
                    }
                },
                number_field::number_field {
                    label: "Total stats",
                    id: "parties.total_stats",
                    value: total_start_stat as i64,
                    disabled: !stat_distribution || !enabled,
                    min: (min_stat * 5) as i64,
                    max: 4995,
                    tooltip: "Total starting stats",
                    onchange: move |x: Event<FormData>| {
                        let new_stat = match x.data.value.parse::<u16>() {
                            Ok(s) => {
                                if s >= min_stat * 5 {
                                    s
                                } else {
                                    total_start_stat
                                }
                            },
                            _ => total_start_stat
                        };

                        state.write().randomizer.parties.total_starting_stats = new_stat
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Res distribution",
                    id: "parties.res_distribution",
                    checked: res_distribution,
                    disabled: !enabled,
                    tooltip: "Randomize player digimon res distribution",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.res_distribution = x.data.value == "true";
                    }
                },
                number_field::number_field {
                    label: "Min res",
                    id: "parties.min_res",
                    value: min_res as i64,
                    disabled: !res_distribution || !enabled,
                    min: 1,
                    max: (total_start_res / 7) as i64,
                    tooltip: "Mininum points per res",
                    onchange: move |x: Event<FormData>| {
                        let new_res = match x.data.value.parse::<u16>() {
                            Ok(s) => {
                                if s * 7 > total_start_res {
                                    min_res
                                } else {
                                    s
                                }
                            },
                            _ => min_res
                        };

                        state.write().randomizer.parties.min_starting_res = new_res
                    }
                },
                number_field::number_field {
                    label: "Total res",
                    id: "parties.total_res",
                    value: total_start_res as i64,
                    disabled: !res_distribution || !enabled,
                    min: (min_res * 7) as i64,
                    max: 6993,
                    tooltip: "Total starting res",
                    onchange: move |x: Event<FormData>| {
                        let new_res = match x.data.value.parse::<u16>() {
                            Ok(s) => {
                                if s >= min_res * 7 {
                                    s
                                } else {
                                    total_start_res
                                }
                            },
                            _ => total_start_res
                        };

                        state.write().randomizer.parties.total_starting_res = new_res
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Stat affinities",
                    checked: stat_affinities,
                    id: "parties.stat_affinites",
                    disabled: !enabled,
                    tooltip: "Randomize stat gain affinities",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.stat_affinities = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Res affinities",
                    checked: res_affinities,
                    id: "parties.res_affinites",
                    disabled: !enabled,
                    tooltip: "Randomize res gain affinities",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.res_affinities = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Learned Tech",
                    checked: learned_tech,
                    id: "parties.learned_tech",
                    disabled: !enabled,
                    tooltip: "Randomize learned tech",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.learned_tech = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Signatures",
                    checked: signatures,
                    id: "parties.signatures",
                    disabled: !enabled,
                    tooltip: "Randomize signature moves",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.signatues = x.data.value == "true";
                    }
                }
            }
        }
    }
}

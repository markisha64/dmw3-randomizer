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

    let learned_tech = read_state.randomizer.parties.learned_tech;
    let signatures = read_state.randomizer.parties.signatures;

    let digivolutions = read_state.randomizer.parties.digivolutions;
    let keep_stages = read_state.randomizer.parties.keep_stages;

    let exp_modifier = read_state.randomizer.parties.exp_modifier;
    let min_exp_mod = read_state.randomizer.parties.min_exp_modifier;
    let max_exp_mod = read_state.randomizer.parties.max_exp_modifier;

    let min_hp_mod = read_state.randomizer.parties.min_hp_modifier;
    let max_hp_mod = read_state.randomizer.parties.max_hp_modifier;
    let min_mp_mod = read_state.randomizer.parties.min_mp_modifier;
    let max_mp_mod = read_state.randomizer.parties.max_mp_modifier;

    let starting_hp_mp = read_state.randomizer.parties.starting_hp_mp;
    let balance_hp_mp = read_state.randomizer.parties.balance_hp_mp;
    let min_start_hp = read_state.randomizer.parties.min_starting_hp;
    let max_start_hp = read_state.randomizer.parties.max_starting_hp;

    let min_start_mp = read_state.randomizer.parties.min_starting_mp;
    let max_start_mp = read_state.randomizer.parties.max_starting_mp;

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
                        state.write().randomizer.parties.signatures = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Digivolutions",
                    checked: digivolutions,
                    id: "parties.digivolutions",
                    disabled: !enabled,
                    tooltip: "Randomize digivolutions",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.digivolutions = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Keep Stages",
                    checked: keep_stages,
                    id: "parties.keep_stages",
                    disabled: !enabled || !digivolutions,
                    tooltip: "Replace digimon of a stage with a digimon of the same stage",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.keep_stages = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                div {
                    class: "tooltip",
                    span {
                        class: "tooltiptext",
                        "Randomize leveling speed"
                    },
                    div {
                        class: "left",
                        checkbox::checkbox {
                            label: "EXP affinity",
                            checked: exp_modifier,
                            id: "parties.exp_modifier",
                            disabled: !enabled,
                            onchange: move |x: Event<FormData>| {
                                state.write().randomizer.parties.exp_modifier = x.data.value == "true";
                            }
                        },
                        number_field::number_field {
                            label: "Min",
                            id: "parties.min_exp_modifier",
                            value: min_exp_mod as i64,
                            disabled: !exp_modifier|| !enabled,
                            min: 1,
                            max: max_exp_mod as i64,
                            onchange: move |x: Event<FormData>| {
                                let new_exp_mod = match x.data.value.parse::<u8>() {
                                    Ok(s) => {
                                        if s >= 1 {
                                            s
                                        } else {
                                            min_exp_mod
                                        }
                                    },
                                    _ => min_exp_mod
                                };

                                state.write().randomizer.parties.min_exp_modifier = new_exp_mod
                            }
                        },
                        number_field::number_field {
                            label: "Max",
                            id: "parties.max_exp_modifier",
                            value: max_exp_mod as i64,
                            disabled: !exp_modifier|| !enabled,
                            min: min_exp_mod as i64,
                            max: 255,
                            onchange: move |x: Event<FormData>| {
                                let new_exp_mod = match x.data.value.parse::<u8>() {
                                    Ok(s) => {
                                        if s >= 1 {
                                            s
                                        } else {
                                            max_exp_mod
                                        }
                                    },
                                    _ => max_exp_mod
                                };

                                state.write().randomizer.parties.max_exp_modifier = new_exp_mod
                            }
                        }
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Starting HP/MP",
                    tooltip: "Randomize starting HP/MP and HP/MP affinity",
                    checked: starting_hp_mp,
                    id: "parties.starting_hp_mp",
                    disabled: !enabled,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.starting_hp_mp = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Balance HP/MP",
                    tooltip: "Balance HP/MP as to be more vanilla like",
                    checked: balance_hp_mp,
                    disabled: !starting_hp_mp || !enabled,
                    id: "parties.balance_hp_mp",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.balance_hp_mp = x.data.value == "true";
                    }
                },
            },
            div {
                class: "left",
                div { style: "margin-right: 10px;", "HP Range" }
                number_field::number_field {
                    label: "Min",
                    id: "parties.min_starting_hp",
                    value: min_start_hp as i64,
                    disabled: !starting_hp_mp|| !enabled,
                    min: 1,
                    max: max_start_hp as i64,
                    onchange: move |x: Event<FormData>| {
                        let new_hp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    min_mp_mod
                                }
                            },
                            _ => min_mp_mod
                        };

                        state.write().randomizer.parties.min_starting_hp = new_hp_mod
                    }
                },
                number_field::number_field {
                    label: "Max",
                    id: "parties.max_starting_hp",
                    value: max_start_hp as i64,
                    disabled: !starting_hp_mp|| !enabled,
                    min: min_start_hp as i64,
                    max: 255,
                    onchange: move |x: Event<FormData>| {
                        let new_hp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    max_mp_mod
                                }
                            },
                            _ => max_mp_mod
                        };

                        state.write().randomizer.parties.max_starting_hp = new_hp_mod
                    }
                }
            },
            div {
                class: "left",
                div { style: "margin-right: 10px;", "MP Range" }
                number_field::number_field {
                    label: "Min",
                    id: "parties.min_starting_mp",
                    value: min_start_mp as i64,
                    disabled: !starting_hp_mp || !enabled,
                    min: 1,
                    max: max_start_mp as i64,
                    onchange: move |x: Event<FormData>| {
                        let new_mp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    min_mp_mod
                                }
                            },
                            _ => min_mp_mod
                        };

                        state.write().randomizer.parties.min_starting_mp = new_mp_mod
                    }
                },
                number_field::number_field {
                    label: "Max",
                    id: "parties.max_starting_mp",
                    value: max_start_mp as i64,
                    disabled: !starting_hp_mp || !enabled,
                    min: min_start_mp as i64,
                    max: 255,
                    onchange: move |x: Event<FormData>| {
                        let new_mp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    max_mp_mod
                                }
                            },
                            _ => max_mp_mod
                        };

                        state.write().randomizer.parties.max_starting_mp = new_mp_mod
                    }
                }
            },
            div {
                class: "left",
                div { style: "margin-right: 10px;", "HP Affinity Range" }
                number_field::number_field {
                    label: "Min",
                    id: "parties.min_hp_modifier",
                    value: min_hp_mod as i64,
                    disabled: !starting_hp_mp || !enabled,
                    min: 1,
                    max: max_hp_mod as i64,
                    onchange: move |x: Event<FormData>| {
                        let new_hp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    min_hp_mod
                                }
                            },
                            _ => min_hp_mod
                        };

                        state.write().randomizer.parties.min_hp_modifier = new_hp_mod
                    }
                },
                number_field::number_field {
                    label: "Max",
                    id: "parties.max_hp_modifier",
                    value: max_hp_mod as i64,
                    disabled: !starting_hp_mp || !enabled,
                    min: min_hp_mod as i64,
                    max: 255,
                    onchange: move |x: Event<FormData>| {
                        let new_hp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    max_hp_mod
                                }
                            },
                            _ => max_hp_mod
                        };

                        state.write().randomizer.parties.max_hp_modifier = new_hp_mod
                    }
                }
            },
            div {
                class: "left",
                div { style: "margin-right: 10px;", "MP Affinity Range" }
                number_field::number_field {
                    label: "Min",
                    id: "parties.min_mp_modifier",
                    value: min_mp_mod as i64,
                    disabled: !starting_hp_mp || !enabled,
                    min: 1,
                    max: max_mp_mod as i64,
                    onchange: move |x: Event<FormData>| {
                        let new_mp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    min_mp_mod
                                }
                            },
                            _ => min_mp_mod
                        };

                        state.write().randomizer.parties.min_mp_modifier = new_mp_mod
                    }
                },
                number_field::number_field {
                    label: "Max",
                    id: "parties.max_mp_modifier",
                    value: max_mp_mod as i64,
                    disabled: !starting_hp_mp || !enabled,
                    min: min_mp_mod as i64,
                    max: 255,
                    onchange: move |x: Event<FormData>| {
                        let new_mp_mod = match x.data.value.parse::<u8>() {
                            Ok(s) => {
                                if s >= 1 {
                                    s
                                } else {
                                    max_mp_mod
                                }
                            },
                            _ => max_mp_mod
                        };

                        state.write().randomizer.parties.max_mp_modifier = new_mp_mod
                    }
                }
            },
        }
    }
}

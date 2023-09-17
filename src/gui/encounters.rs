use dioxus::prelude::*;

use crate::json::TNTStrategy;

use crate::consts;
use crate::json::Preset;

use crate::gui::{checkbox, number_field, slider};

pub fn encounters(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.encounters.enabled;
    let scaling = read_state.randomizer.encounters.scaling;
    let scaling_offset = read_state.randomizer.encounters.scaling_offset;
    let cardmon = read_state.randomizer.encounters.cardmon;
    let bosses = read_state.randomizer.encounters.bosses;
    let keep_zanbamon = read_state.randomizer.encounters.keep_zanbamon;

    let base_stats = read_state.randomizer.encounters.base_stats;
    let base_res = read_state.randomizer.encounters.base_res;
    let stat_modifier = read_state.randomizer.encounters.stat_modifier;
    let res_modifier = read_state.randomizer.encounters.res_modifier;

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Encounters",
                    id: "encounters.enabled",
                    checked: enabled,
                    tooltip: "Shuffle encounters (scales stats)",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.enabled = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Cardmon",
                    id: "encounters.cardmon",
                    checked: cardmon,
                    disabled: !enabled,
                    tooltip: "Keep cardmon unshuffled",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.cardmon = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Bosses",
                    id: "encounters.bosses",
                    checked: bosses,
                    disabled: !enabled,
                    tooltip: "Keep bosses unshuffled",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.bosses = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Keep Zanbamon",
                    id: "encounters.keep_zanbamon",
                    checked: keep_zanbamon,
                    disabled: !enabled,
                    tooltip: "Zanbamon scripted fight can only be won by cheesing it if Zanbamon isn't there",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.keep_zanbamon = x.data.value == "true";
                    }
                },
                div {
                    class: "tooltip",
                    span {
                        class: "tooltiptext",
                        r#style: "max-width: 300px; width: 300px;",
                        "TNT Strategy"
                        br {},
                        "Swap -> swap items with digimon that replaces triceramon"
                        br {},
                        "Keep -> don't move Triceramon",
                        br {},
                        "Shuffle -> fully random"
                    },
                    label {
                        r#for: "encounters.tnt",
                        "TNT strat"
                    },
                    select {
                        id: "encounters.tnt",
                        disabled: !enabled,
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.encounters.strategy = TNTStrategy::from(x.data.value.parse::<u8>().unwrap());
                        },
                        option {
                            value: "2",
                            "Swap"
                        },
                        option {
                            value: "1",
                            "Keep"
                        },
                        option {
                            value: "0",
                            "Shuffle"
                        },
                    }
                }
            },
            div {
                class: "left",
                div {
                    class: "tooltip",
                    span {
                        class: "tooltiptext",
                        style: "width: 350px",
                        "Total stats = Base stats + Stat modifier * level ± [0, Stat range]",
                        br {},
                        "Total res = Base res + Res modifier * level ± [0, Stat range]"
                    },
                    checkbox::checkbox {
                        label: "Scaling",
                        id: "encounters.scaling",
                        checked: scaling,
                        disabled: !enabled,
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.encounters.scaling = x.data.value == "true";
                        }
                    },
                },
                slider::slider {
                    label: "Stat range",
                    value: scaling_offset,
                    id: "encounters.scaling_offset",
                    disabled: !enabled || !scaling,
                    tooltip: "Stat range (undistributed)",
                    oninput: move |x: Event<FormData>| {
                        let new_offset: i64 = match x.data.value.parse::<i64>() {
                            Ok(offset) => {
                                if consts::MIN_STAT_RANGE <= offset && offset <= consts::MAX_STAT_RANGE {
                                    offset
                                } else {
                                    scaling_offset
                                }
                            },
                            _ => scaling_offset
                        };

                        state.write().randomizer.encounters.scaling_offset = new_offset;
                    },
                    min: consts::MIN_STAT_RANGE,
                    max: consts::MAX_STAT_RANGE
                },
            },
            div {
                class: "center",
                number_field::number_field {
                    label: "Base stats",
                    id: "encounters.base_stats",
                    min: 1,
                    max: 2000,
                    value: base_stats as i64,
                    disabled: !enabled || !scaling,
                    onchange: move |x: Event<FormData>| {
                        let stats = match x.data.value.parse::<i32>() {
                            Ok(s) => {
                                if 1 <= s && s <= 2000 {
                                    s
                                } else {
                                    base_stats
                                }
                            },
                            _ => base_stats
                        };

                        state.write().randomizer.encounters.base_stats = stats;
                    },
                },
                number_field::number_field {
                    label: "Base res",
                    id: "encounters.base_res",
                    min: 1,
                    max: 2000,
                    value: base_res as i64,
                    disabled: !enabled || !scaling,
                    onchange: move |x: Event<FormData>| {
                        let res = match x.data.value.parse::<i32>() {
                            Ok(s) => {
                                if 1 <= s && s <= 2000 {
                                    s
                                } else {
                                    base_res
                                }
                            },
                            _ => base_res
                        };

                        state.write().randomizer.encounters.base_res = res;
                    },
                },
                number_field::number_field {
                    label: "Stat modifier",
                    id: "encounters.stat_modifier",
                    min: 1,
                    max: 200,
                    value: stat_modifier as i64,
                    disabled: !enabled || !scaling,
                    onchange: move |x: Event<FormData>| {
                        let modifier = match x.data.value.parse::<i32>() {
                            Ok(s) => {
                                if 1 <= s && s <= 200 {
                                    s
                                } else {
                                    stat_modifier
                                }
                            },
                            _ => stat_modifier
                        };

                        state.write().randomizer.encounters.stat_modifier = modifier;
                    },
                },
                number_field::number_field {
                    label: "Res modifier",
                    id: "encounters.res_modifier",
                    min: 1,
                    max: 200,
                    value: res_modifier as i64,
                    disabled: !enabled || !scaling,
                    onchange: move |x: Event<FormData>| {
                        let modifier = match x.data.value.parse::<i32>() {
                            Ok(s) => {
                                if 1 <= s && s <= 200 {
                                    s
                                } else {
                                    res_modifier
                                }
                            },
                            _ => res_modifier
                        };

                        state.write().randomizer.encounters.res_modifier = modifier;
                    },
                },
            }
        }
    }
}

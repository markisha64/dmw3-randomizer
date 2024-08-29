use crate::gui::{number_field, number_field_float, slider};
use crate::{gui::checkbox, json::Preset};

use dioxus::prelude::*;
use dmw3_consts;

#[component]
pub fn scaling() -> Element {
    let mut preset_state = use_context::<Signal<Preset>>();
    let read_state = preset_state.read();

    let enabled = read_state.scaling.enabled;

    let natural_scaling = read_state.scaling.natural_scaling;

    let scaling_offset = read_state.scaling.scaling_offset;
    let base_stats = read_state.scaling.base_stats;
    let base_res = read_state.scaling.base_res;
    let stat_modifier = read_state.scaling.stat_modifier;
    let res_modifier = read_state.scaling.res_modifier;
    let hp_modifier = read_state.scaling.hp_modifier;

    rsx! {
        div {
            class: "segment",
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
                        id: "scaling.enabled",
                        checked: enabled,
                        onchange: move |x: bool| {
                            preset_state.write().scaling.enabled = x;
                        }
                    },
                },
                slider::slider {
                    label: "Stat range",
                    value: scaling_offset,
                    id: "scaling.scaling_offset",
                    disabled: !enabled,
                    tooltip: "Stat range (undistributed)",
                    oninput: move |x: Event<FormData>| {
                        let new_offset: i64 = match x.data.value().parse::<i64>() {
                            Ok(offset) => {
                                if dmw3_consts::MIN_STAT_RANGE <= offset && offset <= dmw3_consts::MAX_STAT_RANGE {
                                    offset
                                } else {
                                    scaling_offset
                                }
                            },
                            _ => scaling_offset
                        };

                        preset_state.write().scaling.scaling_offset = new_offset;
                    },
                    min: dmw3_consts::MIN_STAT_RANGE,
                    max: dmw3_consts::MAX_STAT_RANGE
                },
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Natural Scaling",
                    id: "scaling.natural_scaling",
                    tooltip: "More natural scaling, scales tech",
                    checked: natural_scaling,
                    onchange: move |x: bool| {
                        preset_state.write().scaling.natural_scaling = x;
                    }
                }
            },
            div {
                class: "center",
                number_field::number_field {
                    label: "Base stats",
                    id: "scaling.base_stats",
                    min: 1,
                    max: 2000,
                    value: base_stats as i64,
                    disabled: !enabled,
                    onchange: move |x: i64| {
                        preset_state.write().scaling.base_stats = x as i32;
                    },
                },
                number_field::number_field {
                    label: "Stat modifier",
                    id: "scaling.stat_modifier",
                    min: 1,
                    max: 200,
                    value: stat_modifier as i64,
                    disabled: !enabled,
                    onchange: move |x: i64| {
                        preset_state.write().scaling.stat_modifier = x as i32;
                    },
                },
            },
            div {
                class: "center",
                number_field::number_field {
                    label: "Base res",
                    id: "scaling.base_res",
                    min: 1,
                    max: 2000,
                    value: base_res as i64,
                    disabled: !enabled,
                    onchange: move |x: i64| {
                        preset_state.write().scaling.base_res = x as i32;
                    },
                },
                number_field::number_field {
                    label: "Res modifier",
                    id: "scaling.res_modifier",
                    min: 1,
                    max: 200,
                    value: res_modifier as i64,
                    disabled: !enabled,
                    onchange: move |x: i64| {
                        preset_state.write().scaling.res_modifier = x as i32;
                    },
                },
            },
            div {
                number_field_float::number_field {
                    min: 0.01,
                    max: 4.0,
                    id: "scaling.hp_modifier",
                    label: "HP modifier",
                    disabled: !enabled,
                    tooltip: "Multiply enemy HP by",
                    onchange: move  |x: f64| {
                        preset_state.write().scaling.hp_modifier = x;
                    },
                    value: hp_modifier
                }
            }
        }
    }
}

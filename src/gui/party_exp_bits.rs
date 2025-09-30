use crate::gui::number_field_float;
use crate::{gui::checkbox, json::Preset};

use dioxus::prelude::*;

#[component]
pub fn party_exp_bits() -> Element {
    let mut preset_state = use_context::<Signal<Preset>>();
    let read_state = preset_state();

    let enabled = read_state.party_exp_bits.enabled;

    let dv_exp_modifier = read_state.party_exp_bits.dv_exp_modifier;
    let exp_modifier = read_state.party_exp_bits.exp_modifier;
    let bits_modifier = read_state.party_exp_bits.bits_modifier;

    let ironmon_genji = read_state.party_exp_bits.ironmon_genji;

    rsx! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Party DV Exp/Exp/Bits",
                    tooltip: "DV Exp/Exp/Bits gains after battle",
                    id: "party_exp_bits.enabled",
                    checked: enabled,
                    onchange: move |x: bool| {
                        preset_state.write().party_exp_bits.enabled = x;
                    }
                },
            }
            div {
                class: "left",
                number_field_float::number_field {
                    min: 0.00,
                    max: 150.00,
                    id: "party_exp_bits.dv_exp_modifier",
                    label: "DV Exp modifer",
                    disabled: !enabled,
                    tooltip: "Multiply DV Exp by",
                    onchange: move |x: f64| {
                        preset_state.write().party_exp_bits.dv_exp_modifier = x;
                    },
                    value: dv_exp_modifier
                }
            }
            div {
                class: "left",
                number_field_float::number_field {
                    min: 0.00,
                    max: 150.00,
                    id: "party_exp_bits.exp_modifier",
                    label: "Exp modifer",
                    disabled: !enabled,
                    tooltip: "Multiply Exp by",
                    onchange: move |x: f64| {
                        preset_state.write().party_exp_bits.exp_modifier = x;
                    },
                    value: exp_modifier
                }
            }
            div {
                class: "left",
                number_field_float::number_field {
                    min: 0.00,
                    max: 150.00,
                    id: "party_exp_bits.bits_modifier",
                    label: "Bits modifer",
                    disabled: !enabled,
                    tooltip: "Multiply Bits by",
                    onchange: move |x: f64| {
                        preset_state.write().party_exp_bits.bits_modifier = x;
                    },
                    value: bits_modifier
                }
            }
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Ironmon Genji",
                    tooltip: "Increases Genji Exp and Bits",
                    id: "party_exp_bits.ironmon_genji",
                    checked: ironmon_genji,
                    disabled: !enabled,
                    onchange: move |x: bool| {
                        preset_state.write().party_exp_bits.ironmon_genji = x;
                    }
                },
            }
       }
    }
}

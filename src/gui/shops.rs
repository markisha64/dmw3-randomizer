use dioxus::prelude::*;

use crate::gui::GlobalState;
use crate::json::Preset;

use crate::gui::checkbox;

pub fn shops(cx: Scope) -> Element {
    let preset_state = use_shared_state::<Preset>(cx).unwrap();
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let read_preset_state = preset_state.read();
    let read_global_state = global_state.read();

    let limit_enabled = read_global_state.shop_limit_enabled;

    let enabled = read_preset_state.randomizer.shops.enabled;

    let limit = match read_preset_state.randomizer.shops.limit_shop_items {
        Some(lm) => lm,
        None => 64,
    };

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Shops",
                    id: "shops.enabled",
                    checked: enabled,
                    tooltip: "Randomize shop items (currently only buyable items)",
                    onchange: move |x: Event<FormData>| {
                        preset_state.write().randomizer.shops.enabled = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left tooltip",
                span {
                    class: "tooltiptext",
                    "Force shop item count",
                },
                checkbox::checkbox {
                    label: "Limit shop items",
                    id: "shops.checkbox",
                    checked: limit_enabled,
                    disabled: !enabled,
                    onchange: move |x: Event<FormData>| {
                        global_state.write().shop_limit_enabled = x.data.value == "true";
                    },
                },
                input {
                    r#type: "number",
                    r#value: "{limit}",
                    disabled: "{!limit_enabled || !enabled}",
                    onchange: move |x| {
                        let limit = match x.data.value.parse::<u8>() {
                            Ok(vl) => vl,
                            _ => 64
                        };

                        preset_state.write().randomizer.shops.limit_shop_items = Some(limit);
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;

use crate::json::Preset;

use crate::gui::checkbox;

pub fn shops(cx: Scope) -> Element {
    let preset_state = use_shared_state::<Preset>(cx).unwrap();
    let read_preset_state = preset_state.read();

    let limit_state = use_state::<bool>(cx, || true);
    let limit_enabled = *limit_state.get();

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
                    onchange: move |x: Event<FormData>| {
                        preset_state.write().randomizer.shops.enabled = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Limit shop items",
                    id: "shops.checkbox",
                    checked: limit_enabled,
                    onchange: move |x: Event<FormData>| {
                         limit_state.modify(|_| x.data.value == "true");
                    },
                },
                input {
                    r#type: "number",
                    r#value: "{limit}",
                    r#disabled: "{!limit_enabled}",
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

use dioxus::prelude::*;

use crate::consts;
use crate::gui::GlobalState;
use crate::json::{Preset, ShopItems};

use crate::gui::checkbox;

pub fn shops(cx: Scope) -> Element {
    let preset_state = use_shared_state::<Preset>(cx).unwrap();
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let read_preset_state = preset_state.read();
    let read_global_state = global_state.read();

    let limit_enabled = read_global_state.shop_limit_enabled;

    let enabled = read_preset_state.randomizer.shops.enabled;
    let selected = read_preset_state.randomizer.shops.items_only.clone();

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
                div {
                    class: "tooltip",
                    span {
                        class: "tooltiptext",
                        "Force shop item count",
                    },
                    div {
                        class: "left",
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
                            class: "short_number",
                            r#type: "number",
                            r#value: "{limit}",
                            disabled: "{!limit_enabled || !enabled}",
                            min: consts::MIN_SHOP_ITEMS,
                            max: consts::MAX_SHOP_ITEMS,
                            onchange: move |x| {
                                let limit = match x.data.value.parse::<u8>() {
                                    Ok(vl) => {
                                        if vl <= 37 {
                                            vl
                                        } else {
                                            limit
                                        }
                                    },
                                    _ => limit
                                };

                                preset_state.write().randomizer.shops.limit_shop_items = Some(limit);
                            }
                        },
                    },
                },
                div {
                    class: "tooltip",
                    span {
                        class: "tooltiptext",
                        style: "width: 200px;",
                        "Buyable => shops contain all buyable items",
                        br {},
                        "Sellable => shops contain all sellable items",
                    },
                    label {
                        r#for: "shops.items_only",
                        "Items"
                    },
                    select {
                        id: "shops.items_only",
                        disabled: !enabled,
                        onchange: move |x: Event<FormData>| {
                            preset_state.write().randomizer.shops.items_only = ShopItems::from(x.data.value.parse::<u8>().unwrap());
                        },
                        option {
                            value: "0",
                            selected: selected == ShopItems::Buyable,
                            "Buyable"
                        },
                        option {
                            value: "1",
                            selected: selected == ShopItems::Sellable,
                            "Sellable"
                        },
                    }
                }
            }
        }
    }
}

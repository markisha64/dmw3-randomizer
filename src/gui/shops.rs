use dioxus::prelude::*;

use crate::consts;
use crate::gui::{number_field, GlobalState};
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

    let sell_price = read_preset_state.randomizer.shops.sell_price;
    let min_sell_price = read_preset_state.randomizer.shops.min_sell_price;
    let max_sell_price = read_preset_state.randomizer.shops.max_sell_price;

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
                },
            },
            div {
                class: "tooltip",
                span {
                    class: "tooltiptext",
                    "Randomize sell price"
                },
                div {
                    class: "left",
                    checkbox::checkbox {
                        id: "shops.min_sell_price",
                        label: "Sell price",
                        disabled: !enabled,
                        checked: sell_price,
                        onchange: move |x: Event<FormData>| {
                            preset_state.write().randomizer.shops.sell_price = x.data.value == "true";
                        },
                    },
                    number_field::number_field {
                        id: "shops.min_sell_price",
                        label: "Min",
                        disabled: !enabled || !sell_price,
                        onchange: move |x: Event<FormData>| {
                            let sell_price = match x.data.value.parse::<i64>(){
                                Ok(price) => {
                                    if consts::MIN_SELL_PRICE <= price && price <= max_sell_price {
                                        price
                                    } else {
                                        min_sell_price
                                    }
                                },
                                _ => min_sell_price
                            };

                            preset_state.write().randomizer.shops.min_sell_price = sell_price;
                        },
                        value: min_sell_price,
                        min: consts::MIN_SELL_PRICE,
                        max: max_sell_price
                    },
                    number_field::number_field {
                        id: "shops.max_sell_price",
                        label: "Min",
                        disabled: !enabled || !sell_price,
                        onchange: move |x: Event<FormData>| {
                            let sell_price = match x.data.value.parse::<i64>(){
                                Ok(price) => {
                                    if min_sell_price <= price && price <= consts::MAX_SELL_PRICE {
                                        price
                                    } else {
                                        max_sell_price
                                    }
                                },
                                _ => max_sell_price
                            };

                            preset_state.write().randomizer.shops.max_sell_price = sell_price;
                        },
                        value: max_sell_price,
                        min: min_sell_price,
                        max: consts::MAX_SELL_PRICE
                    },
                },
            }
        }
    }
}

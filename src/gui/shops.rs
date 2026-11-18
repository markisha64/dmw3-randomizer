use dioxus::prelude::*;

use crate::gui::{help, number_field, theme};
use crate::json::{Preset, ShopItems};

use crate::gui::checkbox;

#[component]
pub fn shops() -> Element {
    let mut preset_state = use_context::<Signal<Preset>>();

    let read_preset_state = preset_state();

    let enabled = read_preset_state.randomizer.shops.enabled;
    let limit_enabled = read_preset_state.randomizer.shops.limit_shop_items_enabled;
    let selected = read_preset_state.randomizer.shops.items_only.clone();

    let limit = read_preset_state.randomizer.shops.limit_shop_items;

    let sell_price = read_preset_state.randomizer.shops.sell_price;
    let min_sell_price = read_preset_state.randomizer.shops.min_sell_price;
    let max_sell_price = read_preset_state.randomizer.shops.max_sell_price;
    let keep_tnt = read_preset_state.randomizer.shops.keep_tnt;
    let healing_ironmon = read_preset_state.randomizer.shops.healing_ironmon;

    rsx! {
        div {
            class: "space-y-3",
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    label: "Shops",
                    id: "shops.enabled",
                    checked: enabled,
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.shops.enabled = x;
                    }
                },
                label {
                    r#for: "shops.items_only",
                    class: theme::LABEL,
                    "Items"
                }
                select {
                    id: "shops.items_only",
                    class: theme::SELECT,
                    disabled: !enabled,
                    onchange: move |x: Event<FormData>| {
                        preset_state.write().randomizer.shops.items_only = ShopItems::from(x.data.value().parse::<u8>().unwrap_or(0));
                    },
                    option {
                        value: "0",
                        selected: selected == ShopItems::Buyable,
                        "Buyable"
                    }
                    option {
                        value: "1",
                        selected: selected == ShopItems::Sellable,
                        "Sellable"
                    }
                    option {
                        value: "2",
                        selected: selected == ShopItems::Ironmon,
                        "Ironmon"
                    },
                }
                help::help {
                    text: "Buyable => shops contain all buyable items\nSellable => shops contain all sellable items"
                }
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    label: "Limit shop items",
                    id: "shops.checkbox",
                    checked: limit_enabled,
                    disabled: !enabled,
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.shops.limit_shop_items_enabled = x;
                    },
                },
                input {
                    class: theme::INPUT_SHORT,
                    r#type: "number",
                    r#value: "{limit}",
                    disabled: "{!limit_enabled || !enabled}",
                    min: dmw3_consts::MIN_SHOP_ITEMS,
                    max: dmw3_consts::MAX_SHOP_ITEMS,
                    onchange: move |x| {
                        let limit = match x.data.value().parse::<u8>() {
                            Ok(vl) => {
                                if vl <= 37 {
                                    vl
                                } else {
                                    limit
                                }
                            },
                            _ => limit
                        };

                        preset_state.write().randomizer.shops.limit_shop_items = limit;
                    }
                },
                help::help { text: "Force shop item count" }
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    id: "shops.min_sell_price",
                    label: "Sell price",
                    disabled: !enabled,
                    checked: sell_price,
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.shops.sell_price = x;
                    },
                },
                checkbox::checkbox {
                    id: "shops.keep_tnt",
                    label: "Keep TNT",
                    disabled: !enabled || !sell_price,
                    checked: keep_tnt,
                    tooltip: "Lock TNT Ball price",
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.shops.keep_tnt = x;
                    },
                },
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                number_field::number_field {
                    id: "shops.min_sell_price",
                    label: "Min",
                    disabled: !enabled || !sell_price,
                    onchange: move |x: i64| {
                        preset_state.write().randomizer.shops.min_sell_price = x;
                    },
                    value: min_sell_price,
                    min: dmw3_consts::MIN_SELL_PRICE,
                    max: max_sell_price
                },
                number_field::number_field {
                    id: "shops.max_sell_price",
                    label: "Max",
                    disabled: !enabled || !sell_price,
                    onchange: move |x: i64| {
                        preset_state.write().randomizer.shops.max_sell_price = x;
                    },
                    value: max_sell_price,
                    min: min_sell_price,
                    max: dmw3_consts::MAX_SELL_PRICE
                },
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    id: "shops.healing_ironmon",
                    label: "Ironmon healing prices",
                    disabled: !enabled,
                    checked: healing_ironmon,
                    tooltip: "Increases the prices of healing items",
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.shops.healing_ironmon = x;
                    },
                },
            }
        }
    }
}

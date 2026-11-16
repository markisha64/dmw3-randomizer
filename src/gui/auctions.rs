use dioxus::prelude::*;

use crate::{
    gui::{checkbox, help, number_field, theme},
    json::{Preset, ShopItems},
};

#[component]
pub fn auctions() -> Element {
    let mut state = use_context::<Signal<Preset>>();
    let read_state = state();

    let enabled = read_state.randomizer.auctions.enabled;

    let auction_items = read_state.randomizer.auctions.auction_items;
    let auction_selected = read_state.randomizer.auctions.auction_items_pool.clone();

    let auction_values = read_state.randomizer.auctions.auction_values;
    let auction_values_min = read_state.randomizer.auctions.auction_values_min as i64;
    let auction_values_max = read_state.randomizer.auctions.auction_values_max as i64;

    rsx! {
        div {
            class: "space-y-3",
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    label: "Auctions",
                    id: "auctions.enabled",
                    checked: enabled,
                    tooltip: "Auctions",
                    onchange: move |x: bool| {
                        state.write().randomizer.auctions.enabled = x;
                    }
                }
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    id: "auctions.auction_items",
                    label: "Auction Items",
                    disabled: !enabled,
                    checked: auction_items,
                    tooltip: "Randomize Auction Items",
                    onchange: move |x: bool| {
                        state.write().randomizer.auctions.auction_items = x;
                    },
                }
                label {
                    r#for: "auctions.auction_items_pool",
                    class: theme::LABEL,
                    "Auction Pool"
                }
                select {
                    id: "auctions.auction_items_pool",
                    class: theme::SELECT,
                    disabled: !enabled || !auction_items,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.auctions.auction_items_pool = ShopItems::from(x.data.value().parse::<u8>().unwrap_or(0));
                    },
                    option {
                        value: "0",
                        selected: auction_selected == ShopItems::Buyable,
                        "Buyable"
                    }
                    option {
                        value: "1",
                        selected: auction_selected == ShopItems::Sellable,
                        "Sellable"
                    }
                    option {
                        value: "2",
                        selected: auction_selected == ShopItems::Ironmon,
                        "Ironmon"
                    },
                }
                help::help {
                    text: "Buyable => all buyable items\nSellable => all sellable items\nIronmon => special pool"
                }
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    id: "auctions.auction_values",
                    label: "Auction Values",
                    disabled: !enabled,
                    checked: auction_values,
                    tooltip: "Randomize Auction Values",
                    onchange: move |x: bool| {
                        state.write().randomizer.auctions.auction_values = x;
                    },
                }
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                number_field::number_field {
                    id: "auctions.auction_values_min",
                    label: "Min",
                    disabled: !enabled || !auction_values,
                    onchange: move |x: i64| {
                        state.write().randomizer.auctions.auction_values_min = x as u32;
                    },
                    value: auction_values_min,
                    min: 100,
                    max: auction_values_max
                },
                number_field::number_field {
                    id: "auctions.auction_values_max",
                    label: "Max",
                    disabled: !enabled || !auction_values,
                    onchange: move |x: i64| {
                        state.write().randomizer.auctions.auction_values_max = x as u32;
                    },
                    value: auction_values_max,
                    min: auction_values_min,
                    max: 999999
                },
            }
        }
    }
}

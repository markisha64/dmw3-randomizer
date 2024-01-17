use dioxus::prelude::*;

use crate::gui::checkbox;
use crate::json::{Preset, ShopItems};

pub fn maps(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.maps.enabled;

    let color = read_state.randomizer.maps.color;
    let backgrounds = read_state.randomizer.maps.backgrounds;
    let item_boxes = read_state.randomizer.maps.item_boxes;
    let selected = read_state.randomizer.maps.item_boxes_items_only.clone();

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Maps",
                    id: "maps.enabled",
                    checked: enabled,
                    tooltip: "Maps",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.maps.enabled = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                div {
                    class: "left",
                    checkbox::checkbox {
                        label: "Color",
                        id: "maps.color",
                        checked: color,
                        disabled: !enabled,
                        tooltip: "Randomize map colorations",
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.maps.color = x.data.value == "true";
                        }
                    },
                },
                div {
                    class: "left",
                    checkbox::checkbox {
                        label: "Backgrounds",
                        id: "maps.backgrounds",
                        checked: backgrounds,
                        disabled: !enabled,
                        tooltip: "Randomize map colorations (can crash)",
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.maps.backgrounds = x.data.value == "true";
                        }
                    },
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Item boxes",
                    id: "maps.item_boxes",
                    checked: item_boxes,
                    disabled: !enabled,
                    tooltip: "Randomize item boxes",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.maps.item_boxes = x.data.value == "true";
                    }
                },
                label {
                    r#for: "maps.item_boxes_items_only",
                    "Items"
                },
                select {
                    id: "maps.item_boxes_items_only",
                    disabled: !enabled,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.maps.item_boxes_items_only = ShopItems::from(x.data.value.parse::<u8>().unwrap());
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

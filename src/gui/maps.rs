use dioxus::prelude::*;

use crate::gui::checkbox;
use crate::json::{GroupStrategy, MusicPool, Preset, ShopItems};

#[component]
pub fn maps() -> Element {
    let mut state = use_context::<Signal<Preset>>();
    let read_state = state();

    let enabled = read_state.randomizer.maps.enabled;

    let color = read_state.randomizer.maps.color;
    let backgrounds = read_state.randomizer.maps.backgrounds;
    let fight_backgrounds = read_state.randomizer.maps.fight_backgrounds;
    let item_boxes = read_state.randomizer.maps.item_boxes;
    let ironmon_charisma = read_state.randomizer.maps.ironmon_charisma;
    let music = read_state.randomizer.maps.music;

    let selected_group_strategy = read_state.randomizer.maps.group_strategy;

    let selected = read_state.randomizer.maps.item_boxes_items_only.clone();

    let music = read_state.randomizer.maps.music;
    let selected_music_pool = read_state.randomizer.maps.music_pool.clone();

    let battle_music = read_state.randomizer.maps.battle_music;
    let selected_battle_music_pool = read_state.randomizer.maps.battle_music_pool.clone();

    rsx! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Maps",
                    id: "maps.enabled",
                    checked: enabled,
                    tooltip: "Maps",
                    onchange: move |x: bool| {
                        state.write().randomizer.maps.enabled = x;
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
                        onchange: move |x: bool| {
                            state.write().randomizer.maps.color = x;
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
                        onchange: move |x: bool| {
                            state.write().randomizer.maps.backgrounds = x;
                        }
                    },
                }
            },
            div {
                class: "left",
                div {
                    checkbox::checkbox {
                        label: "Fight Backgrounds",
                        id: "maps.fight_backgrounds",
                        checked: fight_backgrounds,
                        disabled: !enabled,
                        tooltip: "Randomize fight backgrounds",
                        onchange: move |x: bool| {
                            state.write().randomizer.maps.fight_backgrounds = x;
                        }
                    },
                },
                div {
                    class: "tooltip",
                    span {
                        class: "tooltiptext",
                        style: "width: 200px",
                        "None => fully random",
                        br {},
                        "Map => group based on overworld map",
                        br {},
                        "Party => group based on party id"
                    },
                    label {
                        r#for: "maps.group_strategy",
                        "Group Strategy"
                    },
                    select {
                        id: "maps.group_strategy",
                        disabled: !enabled || !fight_backgrounds,
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.maps.group_strategy = GroupStrategy::from(x.data.value().parse::<u8>().unwrap_or(0));
                        },
                        option {
                            value: "0",
                            selected: selected_group_strategy == GroupStrategy::None,
                            "None"
                        },
                        option {
                            value: "1",
                            selected: selected_group_strategy == GroupStrategy::Map,
                            "Map"
                        },
                        option {
                            value: "2",
                            selected: selected_group_strategy == GroupStrategy::Party,
                            "Party"
                        },
                    }
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
                    onchange: move |x: bool| {
                        state.write().randomizer.maps.item_boxes = x;
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
                        state.write().randomizer.maps.item_boxes_items_only = ShopItems::from(x.data.value().parse::<u8>().unwrap_or(0));
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
                    option {
                        value: "2",
                        selected: selected == ShopItems::Ironmon,
                        "Ironmon"
                    },
                }
            }
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Ironmon charisma",
                    id: "maps.ironmon_charisma",
                    checked: ironmon_charisma,
                    disabled: !enabled,
                    tooltip: "Custom charisma values meant for ironmon",
                    onchange: move |x: bool| {
                        state.write().randomizer.maps.ironmon_charisma = x;
                    }
                },
            }
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Music",
                    id: "maps.randomize_music",
                    checked: music,
                    disabled: !enabled,
                    tooltip: "Randomize Music",
                    onchange: move |x: bool| {
                        state.write().randomizer.maps.music = x;
                    }
                },
                label {
                    r#for: "maps.music_pool",
                    "Music Pool"
                },
                select {
                    id: "maps.music_pool",
                    disabled: !enabled,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.maps.music_pool = MusicPool::from(x.data.value().parse::<u8>().unwrap_or(0));
                    },
                    option {
                        value: "0",
                        selected: selected_music_pool == MusicPool::Overworld,
                        "Overworld"
                    },
                    option {
                        value: "1",
                        selected: selected_music_pool == MusicPool::Battle,
                        "Battle"
                    },
                    option {
                        value: "2",
                        selected: selected_music_pool == MusicPool::Both,
                        "Both"
                    },
                }
            }
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Battle Music",
                    id: "maps.randomize_battle_music",
                    checked: battle_music,
                    disabled: !enabled,
                    tooltip: "Randomize Music",
                    onchange: move |x: bool| {
                        state.write().randomizer.maps.battle_music = x;
                    }
                },
                label {
                    r#for: "maps.battle_music_pool",
                    "Music Pool"
                },
                select {
                    id: "maps.battle_music_pool",
                    disabled: !enabled,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.maps.battle_music_pool = MusicPool::from(x.data.value().parse::<u8>().unwrap_or(0));
                    },
                    option {
                        value: "0",
                        selected: selected_battle_music_pool == MusicPool::Overworld,
                        "Overworld"
                    },
                    option {
                        value: "1",
                        selected: selected_battle_music_pool == MusicPool::Battle,
                        "Battle"
                    },
                    option {
                        value: "2",
                        selected: selected_battle_music_pool == MusicPool::Both,
                        "Both"
                    },
                }
            }
        }
    }
}

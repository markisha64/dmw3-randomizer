use dioxus::prelude::*;

use crate::json::TNTStrategy;

use crate::json::Preset;

use crate::gui::checkbox;

pub fn encounters(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.encounters.enabled;
    let cardmon = read_state.randomizer.encounters.cardmon;
    let bosses = read_state.randomizer.encounters.bosses;

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Encounters",
                    id: "encounters.enabled",
                    checked: enabled,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.enabled = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Cardmon",
                    id: "encounters.cardmon",
                    checked: cardmon,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.cardmon = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Bosses",
                    id: "encounters.bosses",
                    checked: bosses,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.bosses = x.data.value == "true";
                    }
                },
                div {
                    label {
                        r#for: "encounters.tnt",
                        "TNT strat"
                    },
                    select {
                        id: "encounters.tnt",
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.encounters.strategy = TNTStrategy::from(x.data.value.parse::<u8>().unwrap());
                        },
                        option {
                            value: "2",
                            "Swap"
                        },
                        option {
                            value: "1",
                            "Keep"
                        },
                        option {
                            value: "0",
                            "Shuffle"
                        },
                    }
                }
            }
        }
    }
}

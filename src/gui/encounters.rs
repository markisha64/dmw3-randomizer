use dioxus::prelude::*;

use crate::json::TNTStrategy;

use crate::json::Preset;

use crate::gui::checkbox;

pub fn encounters(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.encounters.enabled;
    let scaling = read_state.randomizer.encounters.scaling;
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
                    tooltip: "Shuffle encounters (scales stats)",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.enabled = x.data.value == "true";
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Scaling",
                    id: "encounters.scaling",
                    checked: scaling,
                    disabled: !enabled,
                    tooltip: "Scale encounters",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.scaling = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Cardmon",
                    id: "encounters.cardmon",
                    checked: cardmon,
                    disabled: !enabled,
                    tooltip: "Keep cardmon unshuffled",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.cardmon = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Bosses",
                    id: "encounters.bosses",
                    checked: bosses,
                    disabled: !enabled,
                    tooltip: "Keep bosses unshuffled",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.bosses = x.data.value == "true";
                    }
                },
                div {
                    class: "tooltip",
                    span {
                        class: "tooltiptext",
                        r#style: "max-width: 300px; width: 300px;",
                        "TNT Strategy"
                        br {},
                        "Swap -> swap items with digimon that replaces triceramon"
                        br {},
                        "Keep -> don't move Triceramon",
                        br {},
                        "Shuffle -> fully random"
                    },
                    label {
                        r#for: "encounters.tnt",
                        "TNT strat"
                    },
                    select {
                        id: "encounters.tnt",
                        disabled: !enabled,
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

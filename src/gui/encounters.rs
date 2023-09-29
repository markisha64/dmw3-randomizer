use dioxus::prelude::*;

use crate::json::TNTStrategy;

use crate::json::Preset;

use crate::gui::checkbox;

pub fn encounters(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.encounters.enabled;
    let selected = read_state.randomizer.encounters.strategy.clone();

    let cardmon = read_state.randomizer.encounters.cardmon;
    let bosses = read_state.randomizer.encounters.bosses;
    let keep_zanbamon = read_state.randomizer.encounters.keep_zanbamon;
    let keep_galacticmon = read_state.randomizer.encounters.keep_galacticmon;

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
                    label: "Cardmon",
                    id: "encounters.cardmon",
                    checked: cardmon,
                    disabled: !enabled,
                    tooltip: "Shuffle cardmon",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.cardmon = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Bosses",
                    id: "encounters.bosses",
                    checked: bosses,
                    disabled: !enabled,
                    tooltip: "Shuffle bosses",
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
                            selected: selected == TNTStrategy::Swap,
                            "Swap"
                        },
                        option {
                            value: "1",
                            selected: selected == TNTStrategy::Keep,
                            "Keep"
                        },
                        option {
                            value: "0",
                            selected: selected == TNTStrategy::Shuffle,
                            "Shuffle"
                        },
                    }
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Keep Zanbamon",
                    id: "encounters.keep_zanbamon",
                    checked: keep_zanbamon,
                    disabled: !enabled,
                    tooltip: "Zanbamon scripted fight can only be won by cheesing it if Zanbamon isn't there",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.keep_zanbamon = x.data.value == "true";
                    }
                },
                checkbox::checkbox {
                    label: "Keep Galacticmon",
                    id: "encounters.keep_galacticmon",
                    checked: keep_galacticmon,
                    disabled: !enabled,
                    tooltip: "Galacticmon fight is kinda buggy when it's not Galacticmon",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.encounters.keep_galacticmon = x.data.value == "true";
                    }
                },
            }
        }
    }
}

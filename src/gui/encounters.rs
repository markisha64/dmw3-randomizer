use dioxus::prelude::*;

use super::super::json::Preset;

use super::checkbox;

pub fn encounters(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.encounters.enabled;
    let cardmon = read_state.randomizer.encounters.cardmon;
    let bosses = read_state.randomizer.encounters.bosses;

    render! {
        div {
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Encounters",
                    id: "encounters.enabled",
                    checked: enabled,
                    onchange: |_| {}
                }
            },
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Cardmon",
                    id: "encounters.cardmon",
                    checked: cardmon,
                    onchange: |_| {}
                },
                checkbox::checkbox {
                    label: "Bosses",
                    id: "encounters.bosses",
                    checked: bosses,
                    onchange: |_| {}
                },
            }
        }
    }
}

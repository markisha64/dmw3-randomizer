use dioxus::prelude::*;

use crate::json::Preset;

use crate::gui::checkbox;

pub fn parties(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.party.enabled;
    let random_parties = read_state.randomizer.party.random_parties;

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Party",
                    id: "party.enabled",
                    checked: enabled,
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.party.enabled = x.data.value == "true";
                    }
                },
            },
            div {
                checkbox::checkbox {
                    label: "Parties",
                    id: "parties.random_parties",
                    checked: random_parties,
                    disabled: !enabled,
                    tooltip: "Randomize parties (preview currently unavailable)",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.party.random_parties = x.data.value == "true";
                    }
                },
            }
        }
    }
}

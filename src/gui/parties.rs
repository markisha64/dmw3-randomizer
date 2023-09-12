use dioxus::prelude::*;

use crate::json::Preset;

use crate::gui::checkbox;

pub fn parties(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.parties.enabled;

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Parties",
                    id: "parties.enabled",
                    checked: enabled,
                    tooltip: "Randomize parties (preview currently unavailable)",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.parties.enabled = x.data.value == "true";
                    }
                }
            }
        }
    }
}

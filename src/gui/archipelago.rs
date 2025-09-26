use dioxus::prelude::*;

use crate::gui::checkbox;
use crate::json::Preset;

#[component]
pub fn archipelago() -> Element {
    let mut state = use_context::<Signal<Preset>>();
    let read_state = state();

    let enabled = read_state.archipelago.enabled;

    rsx! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Archipelago",
                    id: "archipelago.enabled",
                    checked: enabled,
                    tooltip: "Output archipelago yaml file",
                    onchange: move |x: bool| {
                        state.write().archipelago.enabled = x;
                    }
                }
            },
        }
    }
}

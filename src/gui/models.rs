use dioxus::prelude::*;

use crate::gui::checkbox;
use crate::json::Preset;

#[component]
pub fn models() -> Element {
    let mut state = use_context::<Signal<Preset>>();
    let read_state = state.read();

    let enabled = read_state.randomizer.models.enabled;

    let model_hue = read_state.randomizer.models.hue_enabled;

    rsx! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Models",
                    id: "models.enabled",
                    checked: enabled,
                    tooltip: "Models",
                    onchange: move |x: bool| {
                        state.write().randomizer.models.enabled = x;
                    }
                }
            },
            div {
                class: "left",
                div {
                    class: "left",
                    checkbox::checkbox {
                        label: "Model hue",
                        id: "models.color",
                        checked: model_hue,
                        disabled: !enabled,
                        tooltip: "Randomize model hue",
                        onchange: move |x: bool| {
                            state.write().randomizer.models.hue_enabled = x;
                        }
                    },
                },
            },
        }
    }
}
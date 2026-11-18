use dioxus::prelude::*;

use crate::gui::checkbox;
use crate::json::Preset;

#[component]
pub fn models() -> Element {
    let mut state = use_context::<Signal<Preset>>();
    let read_state = state();

    let enabled = read_state.randomizer.models.enabled;

    let model_hue = read_state.randomizer.models.hue_enabled;
    let stage_model_hue = read_state.randomizer.models.stage_hue_enabled;

    rsx! {
        div {
            class: "space-y-3",
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    label: "Models",
                    id: "models.enabled",
                    checked: enabled,
                    onchange: move |x: bool| {
                        state.write().randomizer.models.enabled = x;
                    }
                }
            }
            div {
                class: "flex flex-wrap items-center gap-x-5 gap-y-2",
                checkbox::checkbox {
                    label: "Model hue",
                    id: "models.hue",
                    checked: model_hue,
                    disabled: !enabled,
                    onchange: move |x: bool| {
                        state.write().randomizer.models.hue_enabled = x;
                    }
                }
                checkbox::checkbox {
                    label: "Stage model hue",
                    id: "models.stage_hue",
                    checked: stage_model_hue,
                    disabled: !enabled,
                    onchange: move |x: bool| {
                        state.write().randomizer.models.stage_hue_enabled = x;
                    }
                }
            }
        }
    }
}

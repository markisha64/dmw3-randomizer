use dioxus::prelude::*;

use crate::gui::checkbox;
use crate::json::Preset;

pub fn maps(cx: Scope) -> Element {
    let state = use_shared_state::<Preset>(cx).unwrap();
    let read_state = state.read();

    let enabled = read_state.randomizer.maps.enabled;
    let color = read_state.randomizer.maps.color;
    let backgrounds = read_state.randomizer.maps.backgrounds;

    render! {
        div {
            class: "segment",
            div {
                class: "left",
                checkbox::checkbox {
                    label: "Maps",
                    id: "maps.enabled",
                    checked: enabled,
                    tooltip: "Maps",
                    onchange: move |x: Event<FormData>| {
                        state.write().randomizer.maps.enabled = x.data.value == "true";
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
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.maps.color = x.data.value == "true";
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
                        onchange: move |x: Event<FormData>| {
                            state.write().randomizer.maps.backgrounds = x.data.value == "true";
                        }
                    },
                }
            }
        }
    }
}

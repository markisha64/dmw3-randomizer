use dioxus::prelude::*;

use crate::gui::{help, theme};

#[component]
pub fn slider(
    label: &'static str,
    id: &'static str,
    #[props(default = false)] disabled: bool,
    oninput: EventHandler<FormEvent>,
    tooltip: Option<&'static str>,
    value: i64,
    min: i64,
    max: i64,
) -> Element {
    rsx! {
        div {
            class: "flex items-center gap-3",
            label {
                r#for: id,
                class: theme::LABEL,
                "{label}"
            }
            input {
                id: id,
                r#type: "range",
                class: "w-32 accent-sky-300 disabled:cursor-not-allowed disabled:opacity-40",
                disabled: disabled,
                value: value,
                min: min,
                max: max,
                oninput: move |evt| oninput.call(evt)
            }
            input {
                r#type: "number",
                value: value,
                class: theme::INPUT_SHORT,
                min: min,
                max: max,
                disabled: disabled,
                oninput: move |evt| oninput.call(evt)
            }
            if let Some(tooltip) = tooltip {
                help::help { text: tooltip }
            }
        }
    }
}

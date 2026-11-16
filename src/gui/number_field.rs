use dioxus::prelude::*;

use crate::gui::{help, theme};

#[component]
pub fn number_field(
    label: &'static str,
    id: &'static str,
    #[props(default = false)] disabled: bool,
    #[props(default = 1)] step: i64,
    onchange: EventHandler<i64>,
    tooltip: Option<&'static str>,
    value: i64,
    min: i64,
    max: i64,
) -> Element {
    rsx! {
        div {
            class: "flex items-center gap-2",
            label {
                r#for: id,
                class: theme::LABEL,
                "{label}"
            }
            input {
                r#type: "number",
                id: id,
                value: value,
                class: theme::INPUT_SHORT,
                min: min,
                max: max,
                step: step,
                disabled: disabled,
                onchange: move |evt: Event<FormData>| {
                    let value = match evt.data.value().parse::<i64>() {
                        Ok(v) => v,
                        _ => value
                    };

                    onchange.call(value.clamp(min, max));
                }
            }
            if let Some(tooltip) = tooltip {
                help::help { text: tooltip }
            }
        }
    }
}

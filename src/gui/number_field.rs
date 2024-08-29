use dioxus::prelude::*;

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
    let class = match tooltip {
        Some(_) => "tooltip",
        None => "",
    };

    rsx! {
        div {
            class: class,
            label {
                r#for: id,
                "{label}"
            },
            if let Some(tooltip) = tooltip {
                span {
                    class: "tooltiptext",
                    "{tooltip}"
                }
            },
            input {
                r#type: "number",
                value: value,
                class: "short_number",
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
        }
    }
}

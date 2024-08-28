use dioxus::prelude::*;

#[component]
pub fn number_field(
    label: &'static str,
    id: &'static str,
    #[props(default = false)] disabled: bool,
    #[props(default = 0.01)] step: f64,
    onchange: EventHandler<FormEvent>,
    tooltip: Option<&'static str>,
    value: f64,
    min: f64,
    max: f64,
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
                onchange: move |evt| onchange.call(evt)
            }
        }
    }
}

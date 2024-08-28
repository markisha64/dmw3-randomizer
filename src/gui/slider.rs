use dioxus::prelude::*;

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
                id: id,
                r#type: "range",
                class: "slider",
                disabled: disabled,
                value: value,
                min: min,
                max: max,
                oninput: move |evt| oninput.call(evt)
            },
            input {
                r#type: "number",
                value: value,
                class: "short_number",
                min: min,
                max: max,
                disabled: disabled,
                oninput: move |evt| oninput.call(evt)
            }
        }
    }
}

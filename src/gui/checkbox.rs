use dioxus::prelude::*;

#[component]
pub fn checkbox(
    label: &'static str,
    id: &'static str,
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    onchange: EventHandler<FormEvent>,
    tooltip: Option<&'static str>,
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
            }

            input {
                id: id,
                r#type: "checkbox",
                r#checked: checked,
                disabled: disabled,
                onchange: move |evt| onchange.call(evt)
            }
        }
    }
}

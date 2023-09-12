use dioxus::prelude::*;

#[derive(Props)]
pub struct CheckboxProps<'a> {
    label: &'a str,
    id: &'a str,
    #[props(default = false)]
    checked: bool,
    #[props(default = false)]
    disabled: bool,
    onchange: EventHandler<'a, FormEvent>,
    tooltip: Option<&'a str>,
}

pub fn checkbox<'a>(cx: Scope<'a, CheckboxProps<'a>>) -> Element {
    let class = match cx.props.tooltip {
        Some(_) => "tooltip",
        None => "",
    };

    render! {
        div {
            class: class,
            label {
                r#for: cx.props.id,
                cx.props.label
            },
            if let Some(tooltip) = cx.props.tooltip {
                rsx! {
                    span {
                        class: "tooltiptext",
                        tooltip
                    }
                }
            }
            if cx.props.checked {
                rsx! {
                    input {
                        id: cx.props.id,
                        r#type: "checkbox",
                        r#checked: "true",
                        disabled: cx.props.disabled,
                        onchange: move |evt| cx.props.onchange.call(evt)
                    }
                }
            }
            else {
                rsx! {
                    input {
                        id: cx.props.id,
                        r#type: "checkbox",
                        disabled: cx.props.disabled,
                        onchange: move |evt| cx.props.onchange.call(evt)
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;

#[derive(Props)]
pub struct SliderProps<'a> {
    label: &'a str,
    id: &'a str,
    #[props(default = false)]
    disabled: bool,
    onchange: EventHandler<'a, FormEvent>,
    tooltip: Option<&'a str>,
    value: i64,
    min: i64,
    max: i64,
}

pub fn slider<'a>(cx: Scope<'a, SliderProps<'a>>) -> Element {
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
            },
            input {
                id: cx.props.id,
                r#type: "checkbox",
                disabled: cx.props.disabled,
                value: cx.props.value,
                min: cx.props.min,
                max: cx.props.max,
                onchange: move |evt| cx.props.onchange.call(evt)
            },
            "{cx.props.value}"
        }
    }
}

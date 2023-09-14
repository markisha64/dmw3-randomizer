use dioxus::prelude::*;

#[derive(Props)]
pub struct SliderProps<'a> {
    label: &'a str,
    id: &'a str,
    #[props(default = false)]
    disabled: bool,
    oninput: EventHandler<'a, FormEvent>,
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
                r#type: "range",
                class: "slider",
                disabled: cx.props.disabled,
                value: cx.props.value,
                min: cx.props.min,
                max: cx.props.max,
                oninput: move |evt| cx.props.oninput.call(evt)
            },
            input {
                r#type: "number",
                value: cx.props.value,
                class: "short_number",
                min: cx.props.min,
                max: cx.props.max,
                oninput: move |evt| cx.props.oninput.call(evt)
            }
        }
    }
}

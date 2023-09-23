use dioxus::prelude::*;

#[derive(Props)]
pub struct NumberField<'a> {
    label: &'a str,
    id: &'a str,
    #[props(default = false)]
    disabled: bool,
    #[props(default = 1)]
    step: i64,
    onchange: EventHandler<'a, FormEvent>,
    tooltip: Option<&'a str>,
    value: i64,
    min: i64,
    max: i64,
}

pub fn number_field<'a>(cx: Scope<'a, NumberField<'a>>) -> Element {
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
                r#type: "number",
                value: cx.props.value,
                class: "short_number",
                min: cx.props.min,
                max: cx.props.max,
                step: cx.props.step,
                disabled: cx.props.disabled,
                onchange: move |evt| cx.props.onchange.call(evt)
            }
        }
    }
}

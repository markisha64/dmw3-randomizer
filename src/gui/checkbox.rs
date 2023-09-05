use dioxus::prelude::*;

#[derive(Props)]
pub struct CheckboxProps<'a> {
    label: &'a str,
    id: &'a str,
    onchange: EventHandler<'a, FormEvent>,
}

pub fn checkbox<'a>(cx: Scope<'a, CheckboxProps<'a>>) -> Element {
    render! {
        div {
            label {
                r#for: cx.props.id,
                cx.props.label
            },
            input {
                id: cx.props.id,
                r#type: "checkbox",
                onchange: move |evt| cx.props.onchange.call(evt)
            }
        }
    }
}

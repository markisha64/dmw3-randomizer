use dioxus::prelude::*;

#[derive(Props)]
pub struct CheckboxProps<'a> {
    label: &'a str,
    id: &'a str,
    #[props(default = false)]
    checked: bool,
    onchange: EventHandler<'a, FormEvent>,
}

pub fn checkbox<'a>(cx: Scope<'a, CheckboxProps<'a>>) -> Element {
    render! {
        div {
            label {
                r#for: cx.props.id,
                cx.props.label
            },
            if cx.props.checked {
                rsx! {
                    input {
                        id: cx.props.id,
                        r#type: "checkbox",
                        r#checked: "true",
                        onchange: move |evt| cx.props.onchange.call(evt)
                    }
                }
            }
            else {
                rsx! {
                    input {
                        id: cx.props.id,
                        r#type: "checkbox",
                        onchange: move |evt| cx.props.onchange.call(evt)
                    }
                }
            }
        }
    }
}

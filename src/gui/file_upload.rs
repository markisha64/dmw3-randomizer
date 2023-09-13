use dioxus::prelude::*;

#[derive(Props)]
pub struct FileProps<'a> {
    label: &'a str,
    id: &'a str,
    #[props(default = false)]
    disabled: bool,
    #[props(default = false)]
    multiple: bool,
    accept: &'a str,
    onchange: EventHandler<'a, FormEvent>,
    tooltip: Option<&'a str>,
}

pub fn file_upload<'a>(cx: Scope<'a, FileProps<'a>>) -> Element {
    let class = match cx.props.tooltip {
        Some(_) => "center tooltip",
        None => "center",
    };

    render! {
        div {
            class: class,
            label {
                class: "file-upload",
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
            input {
                id: cx.props.id,
                r#type: "file",
                multiple: cx.props.multiple,
                accept: cx.props.accept,
                disabled: cx.props.disabled,
                onchange: move |evt| cx.props.onchange.call(evt)
            }
        }
    }
}

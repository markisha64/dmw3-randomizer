use dioxus::prelude::*;

#[component]
pub fn file_upload(
    label: ReadOnlySignal<String>,
    id: &'static str,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] multiple: bool,
    accept: &'static str,
    onchange: EventHandler<FormEvent>,
    tooltip: Option<&'static str>,
) -> Element {
    let class = match tooltip {
        Some(_) => "center tooltip",
        None => "center",
    };

    rsx! {
        div {
            class: class,
            label {
                class: "file-upload",
                r#for: id,
                "{label}"
            },
            if let Some(tt) = tooltip {
                span {
                    class: "tooltiptext",
                    "{tt}"
                }
            }
            input {
                id: id,
                r#type: "file",
                multiple: multiple,
                accept: accept,
                disabled: disabled,
                onchange: move |evt| onchange.call(evt)
            }
        }
    }
}

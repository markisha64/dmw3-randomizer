use dioxus::prelude::*;

use crate::gui::help;

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
    rsx! {
        div {
            class: "group relative inline-flex items-center gap-2",
            label {
                class: "flex max-w-[20rem] cursor-pointer items-center gap-2 rounded-lg border border-dashed border-white/20 bg-dmw px-3 py-2 text-sm text-white/70 transition hover:border-sky-300/60 hover:bg-white/5 hover:text-white",
                r#for: id,
                span {
                    class: "truncate",
                    "{label}"
                }
            }
            if let Some(tt) = tooltip {
                help::help { text: tt }
            }
            input {
                id: id,
                class: "hidden",
                r#type: "file",
                multiple: multiple,
                accept: accept,
                disabled: disabled,
                onchange: move |evt| onchange.call(evt)
            }
        }
    }
}

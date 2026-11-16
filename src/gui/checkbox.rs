use dioxus::prelude::*;

use crate::gui::help;

#[component]
pub fn checkbox(
    label: &'static str,
    id: &'static str,
    #[props(default = false)] checked: bool,
    #[props(default = false)] disabled: bool,
    onchange: EventHandler<bool>,
    tooltip: Option<&'static str>,
) -> Element {
    let label_class = match disabled {
        true => "cursor-not-allowed select-none text-sm text-white/35",
        false => "cursor-pointer select-none text-sm text-white/75",
    };

    rsx! {
        div {
            class: "flex items-center gap-2",
            input {
                id: id,
                class: "h-4 w-4 shrink-0 cursor-pointer accent-sky-300 disabled:cursor-not-allowed disabled:opacity-40",
                r#type: "checkbox",
                r#checked: checked,
                disabled: disabled,
                onchange: move |evt: Event<FormData>| onchange.call(evt.data.value() == "true")
            }
            label {
                r#for: id,
                class: label_class,
                "{label}"
            }
            if let Some(tooltip) = tooltip {
                help::help { text: tooltip }
            }
        }
    }
}

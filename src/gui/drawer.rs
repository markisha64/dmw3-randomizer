use dioxus::prelude::*;

#[component]
pub fn drawer(
    title: &'static str,
    #[props(default = false)] default_open: bool,
    subtitle: Option<&'static str>,
    children: Element,
) -> Element {
    let mut open = use_signal(|| default_open);
    let is_open = open();
    let chevron = if is_open { "rotate-180" } else { "" };
    let header_radius = if is_open { "rounded-t-xl" } else { "rounded-xl" };
    let aria_expanded = if is_open { "true" } else { "false" };

    rsx! {
        section {
            class: "rounded-xl border border-white/10 bg-dmw/95 shadow-lg shadow-black/30",
            button {
                r#type: "button",
                class: "flex w-full items-center gap-3 px-4 py-3 text-left transition hover:bg-white/5 {header_radius}",
                aria_expanded: "{aria_expanded}",
                onclick: move |_| open.set(!open()),
                span { class: "h-2 w-2 shrink-0 rounded-full bg-dmw-accent shadow-[0_0_10px_rgba(103,232,249,0.7)]" }
                div {
                    class: "min-w-0 flex-1",
                    div {
                        class: "truncate text-sm font-semibold uppercase tracking-wider text-white/90",
                        "{title}"
                    }
                    if let Some(subtitle) = subtitle {
                        div {
                            class: "truncate text-xs text-white/40",
                            "{subtitle}"
                        }
                    }
                }
                span {
                    class: "text-white/40 transition-transform duration-200 {chevron}",
                    "▾"
                }
            }
            if is_open {
                div {
                    class: "border-t border-white/10 px-4 py-4",
                    {children}
                }
            }
        }
    }
}

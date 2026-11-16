use dioxus::prelude::*;

#[component]
pub fn help(text: &'static str) -> Element {
    rsx! {
        span {
            class: "group relative inline-flex items-center",
            span {
                class: "flex h-4 w-4 cursor-help items-center justify-center rounded-full border border-white/20 text-[10px] font-bold leading-none text-white/50 transition group-hover:border-sky-300/60 group-hover:text-sky-300",
                "?"
            }
            span {
                class: "pointer-events-none absolute left-0 top-full z-50 mt-2 hidden w-64 whitespace-pre-line rounded-lg border border-white/15 bg-dmw-panel px-3 py-2 text-left text-xs font-normal leading-relaxed text-white/80 shadow-xl group-hover:block",
                "{text}"
            }
        }
    }
}

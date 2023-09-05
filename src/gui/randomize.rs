use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct RandomizeButton {
    randomizing: bool,
    progress_bar: u8,
}

impl Default for RandomizeButton {
    fn default() -> Self {
        RandomizeButton {
            randomizing: false,
            progress_bar: 0,
        }
    }
}

pub fn randomize(cx: Scope) -> Element {
    let state = use_state::<RandomizeButton>(cx, || RandomizeButton::default());

    let percent = state.progress_bar;

    render! {
        label {
            r#for: "randomize",
            class: "randomize",
            if state.randomizing {
                rsx! {
                    div {
                        r#style: "background-color: #000000; height: 100%; width:{percent}%;"
                    }
                }
            } else {
                rsx! {
                    "Randomize"
                }
            }
        },
        input {
            r#type: "button",
            id: "randomize",
            onclick: |_| {
                let mut current_state = (*state.get()).clone();

                current_state.randomizing = true;

                state.modify(|_| current_state);
            }
        },
    }
}

use dioxus::prelude::*;

use crate::gui::number_field;
use crate::json::Preset;

use crate::gui::checkbox;

#[component]
pub fn card_game() -> Element {
    let mut preset_state = use_context::<Signal<Preset>>();

    let read_preset_state = preset_state();

    let enabled = read_preset_state.randomizer.card_game.enabled;
    let buy_price = read_preset_state.randomizer.card_game.buy_price;
    let min_card_buy_price = read_preset_state.randomizer.card_game.min_card_buy_price;
    let max_card_buy_price = read_preset_state.randomizer.card_game.max_card_buy_price;
    let boosters = read_preset_state.randomizer.card_game.boosters;
    let starting_folder = read_preset_state.randomizer.card_game.starting_folder;

    rsx! {
        div {
            class: "segment",
            div {
                class: "center",
                checkbox::checkbox {
                    label: "Card Game",
                    id: "card_game.enabled",
                    checked: enabled,
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.card_game.enabled = x;
                    }
                },
            },
            div {
                class: "left",
                checkbox::checkbox {
                    id: "card_game.buy_price",
                    label: "Buy price",
                    disabled: !enabled,
                    checked: buy_price,
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.card_game.buy_price = x;
                    },
                },
                checkbox::checkbox {
                    id: "card_game.boosters",
                    label: "Boosters",
                    disabled: !enabled,
                    checked: boosters,
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.card_game.boosters = x;
                    },
                },
            },
            div {
                class: "left",
                number_field::number_field {
                    id: "card_game.min_card_buy_price",
                    label: "Min",
                    disabled: !enabled || !buy_price,
                    onchange: move |x: i64| {
                        preset_state.write().randomizer.card_game.min_card_buy_price = x;
                    },
                    value: min_card_buy_price,
                    min: dmw3_consts::MIN_SELL_PRICE,
                    max: max_card_buy_price
                },
                number_field::number_field {
                    id: "card_game.max_card_buy_price",
                    label: "Max",
                    disabled: !enabled || !buy_price,
                    onchange: move |x: i64| {
                        preset_state.write().randomizer.card_game.max_card_buy_price = x;
                    },
                    value: max_card_buy_price,
                    min: min_card_buy_price,
                    max: dmw3_consts::MAX_SELL_PRICE
                },
            },
            div {
                class: "left",
                checkbox::checkbox {
                    id: "card_game.starting_folder",
                    label: "Starting folder",
                    disabled: !enabled,
                    checked: starting_folder,
                    onchange: move |x: bool| {
                        preset_state.write().randomizer.card_game.starting_folder = x;
                    },
                },
            },
        },
    }
}

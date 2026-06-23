use rand_xoshiro::{rand_core::RngCore, Xoshiro256StarStar};

use crate::{json::Auction, objects::Objects, rand::shops::shoppable};

pub const DEFAULT_AUCTION_PRICES: [[u32; 5]; 16] = [
    [400, 500, 600, 700, 800],
    [400, 500, 600, 700, 800],
    [800, 1100, 1300, 1500, 1600],
    [800, 1100, 1300, 1500, 1600],
    [1600, 2000, 2300, 2600, 2700],
    [1600, 2000, 2300, 2600, 2700],
    [2700, 3100, 3500, 3900, 4000],
    [2700, 3100, 3500, 3900, 4000],
    [4000, 4500, 5500, 5900, 6000],
    [4000, 4500, 5500, 5900, 6000],
    [7000, 8000, 8300, 8600, 8700],
    [7000, 8000, 8300, 8600, 8700],
    [10000, 10900, 11200, 11400, 11500],
    [16000, 16900, 17000, 17400, 17500],
    [23000, 23500, 23800, 23900, 24000],
    [30000, 31000, 31500, 31900, 32000],
];

/// Generate the 6 auction intro strings for file `auction_index * 4`.
///
/// Strings match the in-game pattern:
///   [0] empty
///   [1] Mr. Smith intro (item name, digimon, item type, starting price)
///   [2] Dealer Abel bid
///   [3] Chief Milly bid
///   [4] RPG Fan Masao bid
///   [5] Player hesitation at final price
pub fn auction_intro_text(
    item_name: &str,
    digimon_name: &str,
    item_type: &str,
    prices: &[u32; 5],
) -> [String; 6] {
    [
        String::new(),
        format!(
            "[name]Mr. Smith[name]Now's your chance![pause][clear]Finally, our main item![pause][clear]{}, admired \nby all members![pause][clear]The {} exclusive \n{} will start at\n{} BIT![pause]",
            item_name, digimon_name, item_type, prices[0]
        ),
        format!("[name]Dealer Abel[name]{} BIT![pause]", prices[1]),
        format!("[name]Chief Milly[name]{} BIT![pause]", prices[2]),
        format!("[name]RPG Fan Masao[name]{} BIT![pause]", prices[3]),
        format!(
            "[name][player_name][name]Hmm... {} BIT\n...hmmm...[pause]",
            prices[4]
        ),
    ]
}

/// The unique closing line in each auction's win dialogue (auction 0..14).
pub const AUCTION_WIN_CLOSING: [&str; 15] = [
    "OK, I got a pretty \ncool item, so let's \ngo somewhere.",
    "OK, I got a pretty \ncool item, so I'm \noutta here.",
    "OK, I got a pretty \ncool item, it's time \nto leave.",
    "OK, I got a pretty \ncool item, I'm gonna\ngo check it out.",
    "OK, I got a pretty \ncool item, so let's \nget out of here.",
    "OK, I got a pretty \ncool item, so let's \ngo!",
    "OK, I got a pretty \ncool item, so let's \nhead out.",
    "OK, I got a pretty \ncool item, so let's \nleave this joint.",
    "OK, I got a pretty \ncool item, so let's \ntake off!",
    "OK, I got a pretty \ncool item, so let's \ngo!",
    "OK, I got a pretty \ncool item, so let's \nget out of here.",
    "OK, I got a pretty \ncool item, so let's \ntry it out.",
    "OK, I got a pretty \ncool item, so let's \ntry it out.",
    "OK, I got a pretty \ncool item, so let's \nsee what it does..",
    "OK, I got a pretty \ncool item, so let's \ncheck it out.",
];

/// Generate the 4 bid-menu strings for file `auction_index * 4 + bid_file_offset`.
/// bid_file_offset is 2 for auction 0, 1 for auctions 1-14.
///
///   [0] empty
///   [1] "Bid at N BIT?"
///   [2] "Yes at N BIT." / "Yes, bid." (auction 0 uses different wording)
///   [3] "No, I'll pass."
pub fn auction_bid_text(auction_index: usize, final_price: u32) -> [String; 4] {
    let yes = if auction_index == 0 {
        String::from("Yes, bid.")
    } else {
        format!("Yes at {} BIT.", final_price)
    };
    [
        String::new(),
        format!("Bid at {} BIT?[pause]", final_price),
        format!("{}[pause]", yes),
        String::from("No, I'll pass.[pause]"),
    ]
}

/// Generate the 6 win-dialogue strings.
/// For auction 0 this goes in file 1; for auctions 1-14 it goes in file `auction_index * 4 + 2`.
///
///   [0] empty
///   [1] Player bids
///   [2] Mr. Smith reacts
///   [3] Mr. Smith closes the auction (item sold)
///   [4] Player gets item
///   [5] Player closing remark (unique per auction)
pub fn auction_win_text(item_name: &str, auction_index: usize, final_price: u32) -> [String; 6] {
    let bid_line = if auction_index == 0 {
        format!(
            "[name][player_name][name]All right then!\nI bid {} BIT![pause]",
            final_price
        )
    } else {
        format!(
            "[name][player_name][name]All right then!\n{} BIT![pause]",
            final_price
        )
    };
    [
        String::new(),
        bid_line,
        format!(
            "[name]Mr. Smith[name]Wow! \n{} BIT![pause][clear]Anyone above\n{} BIT?[pause]",
            final_price, final_price
        ),
        format!(
            "[name]Mr. Smith[name]Seems like there's \nno one else.[pause][clear]{} is \nsold for {} BIT![pause][clear]Congratulations\nto you over there![pause][clear]I'm transferring \nthe item to your\nsatellite now![pause]",
            item_name, final_price
        ),
        format!(
            "[name][player_name][name]Yeah! I got \nthe {}![pause]",
            item_name
        ),
        format!(
            "[name][player_name][name]{}[pause]",
            AUCTION_WIN_CLOSING[auction_index]
        ),
    ]
}

/// Generate the 2 pass-dialogue strings for file `auction_index * 4 + 3`.
///
///   [0] empty
///   [1] Player declines
pub fn auction_pass_text() -> [String; 2] {
    [
        String::new(),
        String::from("[name][player_name][name]...it's too much...\n...and I've got \nother stuff I want.[pause][clear]I'll come back when \nI have more money.[pause]"),
    ]
}

/// Oinkmon auction (index 15, files 60-63).
/// The intro oink-speak has prices baked into the gibberish; prices[4] is the player hesitation price.
pub fn oinkmon_intro_text(prices: &[u32; 5]) -> [String; 6] {
    [
        String::new(),
        String::from("[name]Mr. Smith[name]Oin oink oooink![pause][clear]Oink, oin, ooink![pause][clear]Oin, ooink, ooin oi \noiin oinko![pause][clear]Oi oinioi oinoink\noinnk oin oinkk oi\n30000 BIT![pause]"),
        format!("[name]Dealer Abel[name]Oioi!\n{} BIT![pause]", prices[1]),
        format!("[name]Chief Milly[name]Oioi\n{} \nBIT![pause]", prices[2]),
        format!("[name]RPG Fan Masao[name]Oin oi\n{} BIT![pause]", prices[3]),
        format!("[name][player_name][name]Hmm. {} BIT\n...hmmm...[pause]", prices[4]),
    ]
}

pub fn oinkmon_bid_text(final_price: u32) -> [String; 4] {
    [
        String::new(),
        format!("Bid at {} BIT?[pause]", final_price),
        format!("Yes at {} BIT.[pause]", final_price),
        String::from("No, I'll pass.[pause]"),
    ]
}

pub fn oinkmon_win_text(final_price: u32) -> [String; 6] {
    [
        String::new(),
        format!("[name][player_name][name]All right then!\n{} BIT![pause]", final_price),
        format!("[name]Mr. Smith[name]Oink!\n{} BIT![pause][clear]Oinoink, oink\n{} BIT?[pause]", final_price, final_price),
        format!("[name]Mr. Smith[name]Oin oink oioink\noink oioink oink.[pause][clear]Oioi Oink nk\noionko {} BIT![pause][clear]Oinkoinnnk\noi oink oi oiii![pause][clear]Oi oinkoo\noinoiko oin\noioinnnk oink![pause]", final_price),
        String::from("[name][player_name][name]Yeah! I got the\nOioi Oink![pause]"),
        String::from("[name][player_name][name]OK, I got a pretty \ncool item, so let's \nsee what it does.[pause]"),
    ]
}

pub fn oinkmon_pass_text() -> [String; 2] {
    [
        String::new(),
        String::from("[name][player_name][name]...it's too much...\n...and I don't\nknow what it is...[pause][clear]I'll come back when \nI have more money.[pause]"),
    ]
}

pub fn auction_items(preset: &Auction, objects: &mut Objects, rng: &mut Xoshiro256StarStar) {
    let mut pool = shoppable(objects, &preset.auction_items_pool);

    for auction_set in &mut objects.auction_items.modified {
        auction_set.item = pool.remove((rng.next_u64() % pool.len() as u64) as usize);
    }
}

pub fn patch(
    preset: &Auction,
    objects: &mut Objects,
    rng: &mut Xoshiro256StarStar,
) -> anyhow::Result<()> {
    if preset.auction_items {
        auction_items(preset, objects, rng);
    }

    Ok(())
}

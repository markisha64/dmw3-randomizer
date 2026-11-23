use std::iter;

use dmw3_pack::Packed;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Japanese = 0,
    US = 1,
    English = 2,
    French = 3,
    Italian = 4,
    German = 5,
    Spanish = 6,
}

fn encode_raw(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let parsed: dmw3_lang::String = s.parse().unwrap();
    for codepoint in parsed.iter() {
        codepoint.encode(&mut result).unwrap();
    }
    result
}

fn pad(mut v: Vec<u8>) -> Vec<u8> {
    let pad_length = (v.len() / 4 + 1) * 4 - v.len();
    v.extend(iter::repeat(0).take(pad_length));
    v
}

fn encode(s: &str) -> Vec<u8> {
    pad(encode_raw(s))
}

fn encode_with_item(prefix: &str, item_name: &[u8], suffix: &str) -> Vec<u8> {
    let trimmed = &item_name[..item_name.iter().rposition(|&b| b != 0).map_or(0, |i| i + 1)];
    let mut result = encode_raw(prefix);
    result.extend_from_slice(trimmed);
    result.extend(encode_raw(suffix));

    let pad_length = (result.len() / 4 + 1) * 4 - result.len();
    result.extend(iter::repeat(0).take(pad_length));

    result
}

fn packed(files: Vec<Vec<u8>>) -> Packed {
    Packed { files }
}

pub const ENG_WIN_CLOSING: [&str; 15] = [
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

pub const FRA_WIN_CLOSING: [&str; 15] = [
    "Bien, c'est un \nchouette objet, \nallons-y.",
    "Bien, c'est un \nchouette objet, \nje m'en vais.",
    "Bien, c'est un \nchouette objet, \nje m'en vais.",
    "Bon, c'est un \nchouette objet, \nje vais voir ça.",
    "Bon, c'est un \nchouette objet, \nsortons d'ici.",
    "Bon, c'est un \nchouette objet, \nallons-y.",
    "Bon, c'est un \nchouette objet, \nallons-y.",
    "Bon, c'est un \nchouette objet, \nallons-y.",
    "Bon, c'est un \nchouette objet, \non peut y aller.",
    "Bon, c'est un \nchouette objet, \non peut y aller.",
    "Bon, c'est un \nchouette objet, \non peut y aller.",
    "Bon, c'est un \nchouette objet, \nallons l'essayer.",
    "Bon, c'est un \nchouette objet, \non peut y aller.",
    "Bon, c'est un \nchouette objet, \nvoyons son effet.",
    "Bon, c'est un \nchouette objet, \nvoyons ça.",
];

pub const ITA_WIN_CLOSING: [&str; 15] = [
    "Ora che ho un\ncosì bell'oggetto,\ndevo andare.",
    "Ora che ho un così \nbell'oggetto, me\nne andrò.",
    "Ora che ho un così \nbell'oggetto, me ne\nandrò.",
    "Ora che ho un così \nbell'oggetto, posso \nandarmene.",
    "Ora che ho un così \nbell'oggetto, posso \nandarmene.",
    "Ora ho un oggetto\nprezioso. Vado\nvia!",
    "Ora ho un oggetto\nprezioso. Vado\nvia!",
    "Ora ho un oggetto\nprezioso. Posso\nandare via!",
    "Ora ho un oggetto\nprezioso. Posso\nandare!",
    "Ora ho un oggetto\nprezioso. Vado\nvia",
    "Ora ho un oggetto\nprezioso. Posso\nandare via.",
    "Ora ho un oggetto\nprezioso. Vado a\nprovarlo.",
    "Ora ho un oggetto\nprezioso. Vado a\nprovarlo.",
    "Ora ho un oggetto\nprezioso. Vediamo\ncome si comporta.",
    "Ora ho un oggetto\nprezioso. Vediamo\ndi provarlo.",
];

pub const GER_WIN_CLOSING: [&str; 15] = [
    "Ich hab jetzt ein\necht cooles Item,\nalso ziehen wir los.",
    "Ich hab jetzt ein\necht cooles Item,\nalso raus hier.",
    "Ich hab jetzt ein\necht cooles Item.\nZeit, zu gehen.",
    "Ich hab jetzt ein\necht cooles Item,\nich gehe.",
    "Ich hab jetzt ein\necht cooles Item,\nich gehe.",
    "Ich hab jetzt ein\necht cooles Item,\nich geh jetzt.",
    "Ich hab jetzt ein\necht cooles Item,\nalso raus hier.",
    "Ich hab jetzt ein\necht cooles Item,\nich gehe jetzt.",
    "Ich hab jetzt ein\necht cooles Item.\nIch gehe jetzt!",
    "Ich hab jetzt ein\necht cooles Item,\nich gehe jetzt.",
    "Ich hab jetzt ein\necht cooles Item,\nich gehe.",
    "Ich hab jetzt ein\necht cooles Item.\nIch probier's aus.",
    "Ich hab jetzt ein\necht cooles Item.\nIch probier's aus.",
    "Das ist ein echt\ncooles Item. Was\nmach ich damit...",
    "Ich hab jetzt ein\necht cooles Item.\nIch probier's aus.",
];

pub const SPN_WIN_CLOSING: [&str; 15] = [
    "¡Sí!¡Tengo un\nartículo chulo! \nVámonos.",
    "¡Sí!¡Tengo un\nartículo chulo! \n¡Me voy!",
    "¡Sí!¡Tengo un\nartículo chulo! \nHora de irse.",
    "¡Bien! Tengo un\nartículo chulo. ¡Voy\na probarlo!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Salgamos de aquí!",
    "¡Bien! Tengo un\nartículo chulo. \n¡Vamos a probarlo!",
];

impl Language {
    fn to_prefix(&self) -> &str {
        match self {
            Language::Japanese => "M",
            Language::US => "U",
            Language::English => "E",
            Language::French => "F",
            Language::Italian => "I",
            Language::German => "D",
            Language::Spanish => "S",
        }
    }

    pub fn to_folder(&self) -> &str {
        match self {
            Language::Japanese => "JPN",
            Language::US => "USA",
            Language::English => "ENG",
            Language::French => "FRA",
            Language::Italian => "ITA",
            Language::German => "GER",
            Language::Spanish => "SPN",
        }
    }

    pub fn to_received_item(&self, item: Vec<u8>) -> Vec<u8> {
        let mut result = Vec::new();

        let first_not_null = item.len() - item.iter().rev().position(|x| x != &0).unwrap();

        let islice = &item[0..first_not_null];

        let (prefix_str, suffix_str) = match self {
            Language::Japanese => (
                "[name][player_name][name]やった!\n「",
                "」を\nてに，いれたぜ!![pause]",
            ),
            Language::US => ("[name][player_name][name]Yeah! I got\na ", "![pause]"),
            Language::English => ("[name][player_name][name]Yeah! I got\na ", "![pause]"),
            Language::French => ("[name][player_name][name]Ouais ! J'ai\nune ", "![pause]"),
            Language::Italian => ("[name][player_name][name]Sì! Ho una\n", "![pause]"),
            Language::German => ("[name][player_name][name]Yeah! Hab'\neine ", "![pause]"),
            Language::Spanish => ("[name][player_name][name]¡Sí! Tengo\nuna ", "[pause]"),
        };

        let prefix: dmw3_lang::String = prefix_str.parse().unwrap();
        let suffix: dmw3_lang::String = suffix_str.parse().unwrap();

        for codepoint in prefix.iter() {
            codepoint.encode(&mut result).unwrap();
        }

        result.extend_from_slice(islice);
        for codepoint in suffix.iter() {
            codepoint.encode(&mut result).unwrap();
        }

        let pad_length = (result.len() / 4 + 1) * 4 - result.len();
        result.extend(iter::repeat(0).take(pad_length));

        result
    }

    pub fn auction_intro_text(&self, item_name: &[u8], prices: &[u32; 5]) -> Packed {
        match self{
            Language::Japanese => packed(vec![
                encode(""),
                encode_with_item(
                    "[name]オーナー・スミス[name]NOW\nGET A CHANCE!![pause][clear]みなさん! いよいよ，\nこんかいのメインしょうひんの\nとうじょうです!![pause][clear]アイテムきょうかいの\nみなさんの，あこがれ\n「",
                    item_name,
                    &format!("」だ!![pause][clear]{}BITから![pause]", prices[0]),
                ),
                encode(&format!("[name]ディーラーのアベル[name]{}BIT!![pause]", prices[1])),
                encode(&format!("[name]チーフのミリィ[name]{}BIT!![pause]", prices[2])),
                encode(&format!("[name]RPGずきのマサオ[name]{}BIT!![pause]", prices[3])),
                encode(&format!("[name][player_name][name]うーーん，\n{}BITかぁ~\nどうしようかな⋯⋯[pause]", prices[4])),
            ]),
            Language::US | Language::English => packed(vec![
                encode(""),
                encode_with_item(
                    "[name]Mr. Smith[name]Now's your chance![pause][clear]Finally, our main item![pause][clear]",
                    item_name,
                    &format!(", admired \nby all members![pause][clear]Starting at\n{} BIT![pause]", prices[0]),
                ),
                encode(&format!("[name]Dealer Abel[name]{} BIT![pause]", prices[1])),
                encode(&format!("[name]Chief Milly[name]{} BIT![pause]", prices[2])),
                encode(&format!("[name]RPG Fan Masao[name]{} BIT![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hmm... {} BIT\n...hmmm...[pause]", prices[4])),
            ]),
            Language::French => packed(vec![
                encode(""),
                encode_with_item(
                    "[name]M. Smith[name]A vous de jouer ! Voici\nle clou de la vente ![pause][clear]",
                    item_name,
                    &format!(",\nadmirées par tous ![pause][clear]Proposé \nà {} BIT au départ ![pause]", prices[0]),
                ),
                encode(&format!("[name]Marchand Abel[name]{} BIT ![pause]", prices[1])),
                encode(&format!("[name]Chef Milly[name]{} BIT ![pause]", prices[2])),
                encode(&format!("[name]Masao Fan jeu[name]{} BIT ![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hmm. {} BIT\n...hmmm...[pause]", prices[4])),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode_with_item(
                    "[name]Mr. Smith[name]Un'occasione!\nIl pezzo principale![pause][clear]",
                    item_name,
                    &format!(",\nmolto contesa![pause][clear]Si parte\nda {} BIT![pause]", prices[0]),
                ),
                encode(&format!("[name]Cliente Abel[name]{} BIT![pause]", prices[1])),
                encode(&format!("[name]Capo Milly[name]{} BIT![pause]", prices[2])),
                encode(&format!("[name]Masao RPG[name]{} BIT![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hmm... {} BIT\nHmmm...[pause]", prices[4])),
            ]),
            Language::German => packed(vec![
                encode(""),
                encode_with_item(
                    "[name]Mr. Smith[name]Das ist eure Chance!\nEs geht um's Hauptitem![pause][clear]",
                    item_name,
                    &format!(", die alle\nhaben möchten![pause][clear]Mindestgebot:\n{} BIT.[pause]", prices[0]),
                ),
                encode(&format!("[name]Händler Abel[name]{} BIT![pause]", prices[1])),
                encode(&format!("[name]Chief Milly[name]{} BIT![pause]", prices[2])),
                encode(&format!("[name]RPG-Fan Mas.[name]{} BIT![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hm...{} BIT\n...hmmm...[pause]", prices[4])),
            ]),
            Language::Spanish => packed(vec![
                encode(""),
                encode_with_item(
                    "[name]Mr. Smith[name]¡Tu oportunidad!\n¡El artículo principal![pause][clear]",
                    item_name,
                    &format!(".\n¡Admirada por todos![pause][clear]¡Comienza a\n{} BIT![pause]", prices[0]),
                ),
                encode(&format!("[name]Abel el Tendero[name]¡{} BIT![pause]", prices[1])),
                encode(&format!("[name]Jefe Milly[name]¡{} BIT![pause]", prices[2])),
                encode(&format!("[name]Masao RPG[name]¡{} BIT![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Ummm {} BIT\nummm[pause]", prices[4])),
            ]),
        }
    }

    pub fn auction_bid_text(&self, final_price: u32) -> Packed {
        match self {
            Language::Japanese => packed(vec![
                encode(""),
                encode(&format!("{}BITで，さんかしますか?[pause]", final_price)),
                encode(&format!("はい，{}BITでさんかします[pause]", final_price)),
                encode("いいえ，さんかしません[pause]"),
            ]),
            Language::US | Language::English => packed(vec![
                encode(""),
                encode(&format!("Bid at {} BIT?[pause]", final_price)),
                encode(&format!("Yes at {} BIT.[pause]", final_price)),
                encode("No, I'll pass.[pause]"),
            ]),
            Language::French => packed(vec![
                encode(""),
                encode(&format!("Une offre à {} BIT?[pause]", final_price)),
                encode("Oui.[pause]"),
                encode("Non, je passe.[pause]"),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode(&format!("Nessuno a {}?[pause]", final_price)),
                encode(&format!("{} BIT![pause]", final_price)),
                encode("No, grazie.[pause]"),
            ]),
            Language::German => packed(vec![
                encode(""),
                encode(&format!("Gebot bei {} BIT?[pause]", final_price)),
                encode(&format!("Ja, {} BIT.[pause]", final_price)),
                encode("Nein, ich passe.[pause]"),
            ]),
            Language::Spanish => packed(vec![
                encode(""),
                encode(&format!("¿Pujar a {} BIT?[pause]", final_price)),
                encode(&format!("Sí, {} BIT.[pause]", final_price)),
                encode("No, pasar.[pause]"),
            ]),
        }
    }

    pub fn auction_win_text(
        &self,
        item_name: &[u8],
        auction_index: usize,
        final_price: u32,
    ) -> Packed {
        match self {
            Language::Japanese => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]よ~し，しょうぶだ!\nオレは，\n{}BITだぜ![pause]", final_price)),
                encode(&format!("[name]オーナー・スミス[name]おお!\nとうとう，\n{}BITです![pause][clear]{}BITいじょうの\nかたは，いらっしゃい\nますか?[pause]", final_price, final_price)),
                encode_with_item(
                    "[name]オーナー・スミス[name]どうやら，\nほかのかたがたは\nムリのようですねぇ~[pause][clear]それでは，\n「",
                    item_name,
                    &format!("」は\n{}BITで，らくさつ![pause][clear]そこのあなた!\nおめでとう\nございま~す![pause][clear]では，さっそく，\nアイテムをサテライトへ\nてんそうします![pause]", final_price),
                ),
                encode_with_item("[name][player_name][name]やった!\n「", item_name, "」を\nてにいれたぜ![pause]"),
                encode("[name][player_name][name]さて，いいかんじの\nアイテムもてにいれたし，\nほかへ，いこーーっと[pause]"),
            ]),
            Language::US | Language::English => {
                packed(vec![
                    encode(""),
                    encode(&format!("[name][player_name][name]All right then!\n{} BIT![pause]", final_price)),
                    encode(&format!("[name]Mr. Smith[name]Wow! \n{} BIT![pause][clear]Anyone above\n{} BIT?[pause]", final_price, final_price)),
                    encode_with_item(
                        "[name]Mr. Smith[name]Seems like there's \nno one else.[pause][clear]",
                        item_name,
                        &format!(" is \nsold for {} BIT![pause][clear]Congratulations\nto you over there![pause][clear]I'm transferring \nthe item to your\nsatellite now![pause]", final_price),
                    ),
                    encode_with_item("[name][player_name][name]Yeah! I got \nthe ", item_name, "![pause]"),
                    encode(&format!("[name][player_name][name]{}[pause]", ENG_WIN_CLOSING[auction_index])),
                ])
            },
            Language::French => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]Très bien !\n{} BIT ![pause]", final_price)),
                encode(&format!("[name]M. Smith[name]Ouah ! \n{} BIT ![pause][clear]Quelqu'un à \nplus de {} \nBIT ?[pause]", final_price, final_price)),
                encode_with_item(
                    "[name]M. Smith[name]Il semble qu'il\nn'y ait personne.[pause][clear]",
                    item_name,
                    &format!("\nest vendue {} \nBIT ![pause][clear]Félicitations au\nMonsieur là-bas ![pause][clear]Je transfère de\nsuite l'objet sur \nvotre satellite ![pause]", final_price),
                ),
                encode_with_item("[name][player_name][name]Oui ! J'ai \n", item_name, " ![pause]"),
                encode(&format!("[name][player_name][name]{}[pause]", FRA_WIN_CLOSING[auction_index])),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]Va bene!\n{} BIT![pause]", final_price)),
                encode(&format!("[name]Mr. Smith[name]Wow!\n{} BIT![pause][clear]Nessuno offre\ndi più?[pause]", final_price)),
                encode_with_item(
                    "[name]Mr. Smith[name]Sembra proprio\ndi no.[pause][clear]",
                    item_name,
                    &format!("\nper {} BIT![pause][clear]Congratulazioni\na te, laggiù![pause][clear]Trasferirò \nl'oggetto al\nsatellite![pause]", final_price),
                ),
                encode_with_item("[name][player_name][name]Ho \n", item_name, "![pause]"),
                encode(&format!("[name][player_name][name]{}[pause]", ITA_WIN_CLOSING[auction_index])),
            ]),
            Language::German => {
                packed(vec![
                    encode(""),
                    encode(&format!("[name][player_name][name]Also\n{} BIT![pause]", final_price)),
                    encode(&format!("[name]Mr. Smith[name]Wow!\n{} BIT![pause][clear]Bietet jmd.\nmehr?[pause]", final_price)),
                    encode_with_item(
                        "[name]Mr. Smith[name]Es bietet wohl\nkeiner mehr.[pause][clear]",
                        item_name,
                        &format!(" \nfür {} BIT![pause][clear]Gratulation![pause][clear]Ich übertrage das\nItem jetzt an \ndeinen Satelliten![pause]", final_price),
                    ),
                    encode_with_item("[name][player_name][name]Super! \n", item_name, "![pause]"),
                    encode(&format!("[name][player_name][name]{}[pause]", GER_WIN_CLOSING[auction_index])),
                ])
            },
            Language::Spanish => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]¡De acuerdo!\n¡Pujo por {}![pause]", final_price)),
                encode(&format!("[name]Sr. Smith[name]¡Uauu!\n¡{} BIT![pause][clear]¿Alguien sobre\n{} BIT?[pause]", final_price, final_price)),
                encode_with_item(
                    "[name]Sr. Smith[name]Parece que no hay\nnadie más.[pause][clear]¡",
                    item_name,
                    &format!("\nvendida, {} BIT![pause][clear]¡Felicidades\nal chico de ahí![pause][clear]¡Estoy transfiriendo \nel artículo a\ntu satélite![pause]", final_price),
                ),
                encode_with_item("[name][player_name][name]¡Sí! ¡Tengo \n", item_name, "![pause]"),
                encode(&format!("[name][player_name][name]{}[pause]", SPN_WIN_CLOSING[auction_index])),
            ]),
        }
    }

    pub fn auction_pass_text(&self) -> Packed {
        match self {
            Language::Japanese => packed(vec![
                encode(""),
                encode("⋯かえないよなぁ~\n⋯それに，ほかにほしい\nモノもあるしなぁ~[pause][clear]もうちょっと，おかね\nためてから，また\nこよぉーーっと[pause]"),
            ]),
            Language::US | Language::English => packed(vec![
                encode(""),
                encode("[name][player_name][name]...it's too much...\n...and I've got \nother stuff I want.[pause][clear]I'll come back when \nI have more money.[pause]"),
            ]),
            Language::French => packed(vec![
                encode(""),
                encode("[name][player_name][name]C'est trop...\n...et je veux \nd'autres objets.[pause][clear]Je reviendrai quand \nj'aurai plus \nd'argent.[pause]"),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode("[name][player_name][name]Costa troppo,\ne c'è altra\nroba che voglio.[pause][clear]Tornerò quando\navrò i soldi.[pause]"),
            ]),
            Language::German => packed(vec![
                encode(""),
                encode("[name][player_name][name]Das ist zu viel...\nIch möchte noch ein\npaar andere Sachen[pause][clear]Ich komme mit \nmehr Geld zurück.[pause]"),
            ]),
            Language::Spanish => packed(vec![
                encode(""),
                encode("[name][player_name][name]Es demasiado...\nY hay otras cosas \nque quiero...[pause][clear]Volveré cuando\ntenga más dinero.[pause]"),
            ]),
        }
    }

    /// Oinkmon auction (index 15, files 60-63).
    pub fn oinkmon_intro_text(&self, prices: &[u32; 5]) -> Packed {
        match self {
            Language::Japanese => packed(vec![
                encode(""),
                encode(&format!("[name]オーナー・スミス[name]ブイ\nブッブィ ブブブィ-!![pause][clear]ブーッ! ブイブイ，\nブヒッブイブイー\nブイ ブブッブィ!![pause][clear]ブイッブイブブイ\nブイブイブー，ブイブイ\n「ブイブイブーブー」ブッ!![pause][clear]ブブブヒブイーブ\nブイブーブィ，\n{}BITブイ![pause]", prices[0])),
                encode(&format!("[name]ディーラーのアベル[name]ブイブイ!\n{}BIT!![pause]", prices[1])),
                encode(&format!("[name]チーフのミリィ[name]ブッブイ\n{}BIT!![pause]", prices[2])),
                encode(&format!("[name]RPGずきのマサオ[name]ブーブー\n{}BIT!![pause]", prices[3])),
                encode(&format!("[name][player_name][name]うーーん，\n{}BITかぁ~\nどうしようかな⋯⋯[pause]", prices[4])),
            ]),
            Language::US | Language::English => packed(vec![
                encode(""),
                encode(&format!("[name]Mr. Smith[name]Oin oink oooink![pause][clear]Oink, oin, ooink![pause][clear]Oin, ooink, ooin oi \noiin oinko![pause][clear]Oi oinioi oinoink\noinnk oin oinkk oi\n{} BIT![pause]", prices[0])),
                encode(&format!("[name]Dealer Abel[name]Oioi!\n{} BIT![pause]", prices[1])),
                encode(&format!("[name]Chief Milly[name]Oioi\n{} \nBIT![pause]", prices[2])),
                encode(&format!("[name]RPG Fan Masao[name]Oin oi\n{} BIT![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hmm. {} BIT\n...hmmm...[pause]", prices[4])),
            ]),
            Language::French => packed(vec![
                encode(""),
                encode(&format!("[name]M. Smith[name]Oin oink oooink ![pause][clear]Oink, oin, ooink ![pause][clear]Oin, ooink, ooin oi \noiin oinko ![pause][clear]Oi oinioi oinoink\noinnk oin oinkk oi\n{} BIT ![pause]", prices[0])),
                encode(&format!("[name]Marchand Abel[name]Oioi !\n{} BIT ![pause]", prices[1])),
                encode(&format!("[name]Chef Milly[name]Oioi\n{} \nBIT ![pause]", prices[2])),
                encode(&format!("[name]Masao Fan jeu[name]Oin oi\n{} BIT ![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hmm. {} \nBIT...hmmm...[pause]", prices[4])),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode(&format!("[name]Mr. Smith[name]Oin oink oooink![pause][clear]Oink, oin, ooink![pause][clear]Oin, ooink, ooin oi \noiin oinko![pause][clear]Oi oinioi oinoink\noinnk oin oinkk oi\n{} BIT![pause]", prices[0])),
                encode(&format!("[name]Cliente Abel[name]Oioi!\n{} BIT![pause]", prices[1])),
                encode(&format!("[name]Capo Milly[name]Oioi\n{} \nBIT![pause]", prices[2])),
                encode(&format!("[name]Masao RPG[name]Oin oi\n{} BIT![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hm, {} BIT\nMmmm...[pause]", prices[4])),
            ]),
            Language::German => packed(vec![
                encode(""),
                encode(&format!("[name]Mr. Smith[name]Oin oink oooink![pause][clear]Oink, oin, ooink![pause][clear]Oin, ooink, ooin oi \noiin oinko![pause][clear]Oi oinioi oinoink\noinnk oin oinkk oi\n{} BIT![pause]", prices[0])),
                encode(&format!("[name]Händler Abel[name]Oioi!\n{} BIT![pause]", prices[1])),
                encode(&format!("[name]Chief Milly[name]Oioi\n{} \nBIT![pause]", prices[2])),
                encode(&format!("[name]RPG-Fan Mas.[name]Oin oi\n{} BIT![pause]", prices[3])),
                encode(&format!("[name][player_name][name]Hm. {} BIT\n...hmmm...[pause]", prices[4])),
            ]),
            Language::Spanish => packed(vec![
                encode(""),
                encode(&format!("[name]Sr. Smith[name]¡Oin oink oooink![pause][clear]¡Oink, oin, ooink![pause][clear]¡Oin, ooink, ooin oi \noiin oinko![pause][clear]¡Oi oinioi oinoink\noinnk oin oinkk oi\n{} BIT![pause]", prices[0])),
                encode(&format!("[name]Abel el Tendero[name]¡Oioi!\n¡{} BIT![pause]", prices[1])),
                encode(&format!("[name]Jefe Milly[name]¡Oioi\n{}\nBIT![pause]", prices[2])),
                encode(&format!("[name]Masao RPG[name]Oin oi\n{} BIT[pause]", prices[3])),
                encode(&format!("[name][player_name][name]Um. {} BIT\n...ummm...[pause]", prices[4])),
            ]),
        }
    }

    pub fn oinkmon_bid_text(&self, final_price: u32) -> Packed {
        match self {
            Language::Japanese => packed(vec![
                encode(""),
                encode(&format!("{}BITで，さんかしますか?[pause]", final_price)),
                encode(&format!("はい，{}BITでさんかします[pause]", final_price)),
                encode("いいえ，さんかしません[pause]"),
            ]),
            Language::US | Language::English => packed(vec![
                encode(""),
                encode(&format!("Bid at {} BIT?[pause]", final_price)),
                encode(&format!("Yes at {} BIT.[pause]", final_price)),
                encode("No, I'll pass.[pause]"),
            ]),
            Language::French => packed(vec![
                encode(""),
                encode(&format!("Une offre à {} BIT?[pause]", final_price)),
                encode(&format!("Oui, {} BIT.[pause]", final_price)),
                encode("Non, je passe.[pause]"),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode(&format!("Nessuno a {}?[pause]", final_price)),
                encode(&format!("Sì, {} BIT.[pause]", final_price)),
                encode("No, grazie.[pause]"),
            ]),
            Language::German => packed(vec![
                encode(""),
                encode(&format!("Gebot bei {} BIT?[pause]", final_price)),
                encode(&format!("Ja bei {} BIT![pause]", final_price)),
                encode("Nein, ich passe.[pause]"),
            ]),
            Language::Spanish => packed(vec![
                encode(""),
                encode(&format!("¿Pujar a {}BIT?[pause]", final_price)),
                encode(&format!("Sí, a {} BIT.[pause]", final_price)),
                encode("No, pasar[pause]"),
            ]),
        }
    }

    pub fn oinkmon_win_text(&self, final_price: u32) -> Packed {
        match self{
            Language::Japanese => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]よ~し，しょうぶだ!\nオレは，\n{}BITだぜ![pause]", final_price)),
                encode(&format!("[name]オーナー・スミス[name]ブー!\nブイブイ，\n{}BITブー![pause][clear]{}BITッブイ\nブィー，ブイブイブッ\nブイブイ?[pause]", final_price, final_price)),
                encode(&format!("[name]オーナー・スミス[name]ブイブイ，\nブーブブブィ\nブブブィブイッブ~[pause][clear]ブイブイ，\n「ブイブイブーブー」ブィ\n{}BITッブ，ブィー![pause][clear]ブイブイブイブイ!\nブイッブイッ\nブイブイ~ブ~![pause][clear]ブイ，ブッブィ，\nブイブイッブイブイブー\nブイブーブィ~![pause]", final_price)),
                encode("[name][player_name][name]やった!!\nブイブイブーブーを\nてにいれたぜ![pause]"),
                encode("[name][player_name][name]さて，いいかんじの\nアイテムもてにいれたし，\nほかへ，いこーーっと[pause]"),
            ]),
            Language::US | Language::English => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]All right then!\n{} BIT![pause]", final_price)),
                encode(&format!("[name]Mr. Smith[name]Oink!\n{} BIT![pause][clear]Oinoink, oink\n{} BIT?[pause]", final_price, final_price)),
                encode(&format!("[name]Mr. Smith[name]Oin oink oioink\noink oioink oink.[pause][clear]Oioi Oink nk\noionko {} BIT![pause][clear]Oinkoinnnk\noi oink oi oiii![pause][clear]Oi oinkoo\noinoiko oin\noioinnnk oink![pause]", final_price)),
                encode("[name][player_name][name]Yeah! I got the\nOioi Oink![pause]"),
                encode("[name][player_name][name]OK, I got a pretty \ncool item, so let's \nsee what it does.[pause]"),
            ]),
            Language::French => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]Très bien ! \n{} BIT ![pause]", final_price)),
                encode(&format!("[name]M. Smith[name]Oink ! \n{} BIT ![pause][clear]Oinoink, oink \n{} BIT ?[pause]", final_price, final_price)),
                encode(&format!("[name]M. Smith[name]Oin oink oioink\noink oioink oink.[pause][clear]Oioi Oink nk\noionko {} BIT [pause][clear]Oinkoinnnk\noi oink oi oiii ![pause][clear]Oi oinkoo\noinoiko oin\noioinnnk oink ![pause]", final_price)),
                encode("[name][player_name][name]Oui ! J'ai \nl'Oioi Oink ![pause]"),
                encode("[name][player_name][name]Bon, c'est un \nchouette objet, \nvoyons ça.[pause]"),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]Va bene!\n{} BIT![pause]", final_price)),
                encode(&format!("[name]Mr. Smith[name]Oink!\n{} BIT![pause][clear]Oinoink, oink\n{} BIT?[pause]", final_price, final_price)),
                encode(&format!("[name]Mr. Smith[name]Oin oink oioink\noink oioink oink.[pause][clear]Oioi Oink nk\noionko {} BIT![pause][clear]Oinkoinnnk\noi oink oi oiii![pause][clear]Oi oinkoo\noinoiko oin\noioinnnk oink![pause]", final_price)),
                encode("[name][player_name][name]Wow! Ho la\nOioi Oink![pause]"),
                encode("[name][player_name][name]Ora ho un oggetto\nprezioso. Vediamo\ndi provarlo.[pause]"),
            ]),
            Language::German => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]Also\n{} BIT![pause]", final_price)),
                encode(&format!("[name]Mr. Smith[name]Oink!\n{} BIT![pause][clear]Oinoink, oink\n{} BIT?[pause]", final_price, final_price)),
                encode(&format!("[name]Mr. Smith[name]Oin oink oioink\noink oioink oink.[pause][clear]Oioi Oink nk\noionko {} BIT![pause][clear]Oinkoinnnk\noi oink oi oiii![pause][clear]Oi oinkoo\noinoiko oin\noioinnnk oink![pause]", final_price)),
                encode("[name][player_name][name]Yeah! \nOioi Oink![pause]"),
                encode("[name][player_name][name]Das ist ein echt\ncooles Item. Was\nmach ich damit...[pause]"),
            ]),
            Language::Spanish => packed(vec![
                encode(""),
                encode(&format!("[name][player_name][name]¡De acuerdo!\n¡{} BIT![pause]", final_price)),
                encode(&format!("[name]Sr. Smith[name]Oink\n{} BIT[pause][clear]Oinoink, oink\n¿{} BIT?[pause]", final_price, final_price)),
                encode(&format!("[name]Sr. Smith[name]Oin oink oioink\noink oioink oink.[pause][clear]¡Oioi Oink nk\noionko {} BIT![pause][clear]¡Oinkoinnnk\noi oink oi oiii![pause][clear]¡Oi oinkoo\noinoiko oin\noioinnnk oink![pause]", final_price)),
                encode("[name][player_name][name]¡Sí!¡Tengo el\nOioi Oink![pause]"),
                encode("[name][player_name][name]Ya tengo un artículo\nchulo. Vamos a ver\nqué hace.[pause]"),
            ]),
        }
    }

    pub fn oinkmon_pass_text(&self) -> Packed {
        match self {
            Language::Japanese => packed(vec![
                encode(""),
                encode("⋯かえないよなぁ~\n⋯それに，どんなアイテムか\nわかんないしなぁ~[pause][clear]もうちょっと，おかね\nためてから，また\nこよぉーーっと[pause]"),
            ]),
            Language::French => packed(vec![
                encode(""),
                encode("[name][player_name][name]C'est trop...\n...et je veux \nd'autres objets.[pause][clear]Je reviendrai quand \nj'aurai plus \nd'argent.[pause]"),
            ]),
            Language::Italian => packed(vec![
                encode(""),
                encode("[name][player_name][name]Costa troppo,\ne c'è altra\nroba che voglio.[pause][clear]Tornerò quando\navrò i soldi.[pause]"),
            ]),
            Language::German => packed(vec![
                encode(""),
                encode("[name][player_name][name]Das ist zu viel...\nIch möchte noch ein\npaar andere Sachen.[pause][clear]Ich komme mit \nmehr Geld zurück.[pause]"),
            ]),
            Language::Spanish => packed(vec![
                encode(""),
                encode("[name][player_name][name]...Es demasiado...\nY no se qué es...[pause][clear]Volveré cuando \ntenga más dinero.[pause]"),
            ]),
            _ => packed(vec![
                encode(""),
                encode("[name][player_name][name]...it's too much...\n...and I don't\nknow what it is...[pause][clear]I'll come back when \nI have more money.[pause]"),
            ]),
        }
    }
    pub fn to_file_name(&self, file_name: &str) -> String {
        format!("{}{}", self.to_prefix(), file_name)
    }

    pub fn to_path(&self, file_name: &str) -> String {
        format!(
            "AAA/DAT/COUNTRY/{}/{}",
            self.to_folder(),
            self.to_file_name(file_name)
        )
    }
}

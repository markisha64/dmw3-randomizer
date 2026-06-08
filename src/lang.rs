use std::iter;

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
            Language::Japanese => ("[name][player_name][name]やった!\n「", "」を\nてに，いれたぜ!![pause]"),
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

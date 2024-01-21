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

    pub fn to_received_item_generic(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // I ain't making this for all languages, if someone want to go ahead
        match self {
            _ => {
                result.extend(b"\x02\x07\x02\x09\x02\x07\x26\x2C\x28\x2F\xE7\x01\x01\x16\x01\x01\x2E\x36\x3B\x01\x01\x28\x35\x01\x01\x30\x3b\x2c\x34\xe7\x02\x02\x02");
            }
        }

        let pad_length = (result.len() / 4 + 1) * 4 - result.len();
        result.extend(iter::repeat(0).take(pad_length));

        result
    }

    pub fn to_received_item(&self, item: Vec<u8>) -> Vec<u8> {
        let mut result = Vec::new();

        let first_not_null = item.len() - item.iter().rev().position(|x| x != &0).unwrap();

        let islice = &item[0..first_not_null];

        // I ain't making this for all languages, if someone want to go ahead
        match self {
            _ => {
                result.extend(b"\x02\x07\x02\x09\x02\x07\x26\x2C\x28\x2F\xE7\x01\x01\x16\x01\x01\x2E\x36\x3B\x01\x01\x28\x01\x01\x02\x01");
                result.extend(islice);
                result.extend(b"\xE7\x02\x02\x02");
            }
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

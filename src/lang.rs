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
            Language::German => "G",
            Language::Spanish => "S",
        }
    }

    fn to_folder(&self) -> &str {
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

    fn to_file_name(&self, file_name: &str) -> String {
        return format!("{}{}", self.to_prefix(), file_name);
    }
}

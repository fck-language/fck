pub mod keywords;
pub mod en;
pub mod fr;
pub mod de;

pub fn get_associated_keywords(lang_code: &str) -> Option<keywords::Keywords<'static>> {
    match lang_code {
        "en" => Some(en::KEYWORDS),
        "de" => Some(de::KEYWORDS),
        "fr" => Some(fr::KEYWORDS),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

pub fn get_associated_messages(lang_code: &str) -> Option<keywords::Messages<'static>> {
    match lang_code {
        "en" => Some(en::MESSAGES),
        "de" => Some(de::MESSAGES),
        "fr" => Some(fr::MESSAGES),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

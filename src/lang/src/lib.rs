pub mod keywords;
pub mod en;
pub mod fr;
pub mod de;

pub fn get_associated_keywords(lang_code: &str) -> Option<keywords::Keywords> {
    match lang_code {
        "en" => Some(en::KEYWORDS_EN),
        "de" => Some(de::KEYWORDS_DE),
        "fr" => Some(fr::KEYWORDS_FR),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

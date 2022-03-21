//! Language handling
//!
//! This contains all the keywords currently available for fck, as well as the functions that allow
//! the extended use of them. This crate (in the fck repo) also includes the `fckl equivalents`
//! folder that has the `.fckl` language files equivalent to all the currently included languages
pub mod keywords;
pub mod en;
pub mod fr;
pub mod de;
pub mod ko;

/// Returns (assuming the language code is valid) the keywords relating to the given language code
pub fn get_associated_keywords(lang_code: &str) -> Option<keywords::Keywords<'static>> {
    match lang_code {
        "en" => Some(en::KEYWORDS),
        "de" => Some(de::KEYWORDS),
        "fr" => Some(fr::KEYWORDS),
        "ko" => Some(ko::KEYWORDS),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

/// Returns (assuming the language code is valid) the messages relating to the given language code
pub fn get_associated_messages(lang_code: &str) -> Option<keywords::Messages<'static>> {
    match lang_code {
        "en" => Some(en::MESSAGES),
        "de" => Some(de::MESSAGES),
        "fr" => Some(fr::MESSAGES),
        "ko" => Some(ko::MESSAGES),
        t => {
            println!("Unknown language code \"{}\"", t);
            None
        }
    }
}

/// Custom format macro
///
/// Used similarly to the normal `format!` macro, with some alterations. The first variable can be
/// a `&str` or `String` and can be a variable. Any `_` is replaced with the index appropriate
/// value. All formatting is the `std::fmt::Display` not `std::fmt::Debug` so beware. this is
/// required to be able to use language specific string formats that can be formatted such as error
/// messages with information embedded in them. **DO NOT CALL THIS WITH ONLY A FORMAT STRING**
#[macro_export]
macro_rules! fmt {
    ($f: expr, $($a: expr), *) => {{
        let mut out = String::new();
		let F = format!(" {} ", $f);
		let mut iter = F.split('_');
		$(
		out = format!("{}{}{}", out, iter.next().unwrap(), $a);
		)*
		out = format!("{}{}", out, iter.next().unwrap());
        out.get(1..out.len() - 1).unwrap().to_string()
    }}
}

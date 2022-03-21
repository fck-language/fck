//! Structs for the languages
//!
//! These are the structs for the keywords and messages (anything language based) that all the other
//! files rely on

/// Keyword struct
///
/// Contains all the various keywords that are used
#[derive(Clone, Copy)]
pub struct Keywords<'a> {
    /// General keywords
    ///
    /// List index 0
    pub keywords: [&'a str; 22],
    /// Variable keywords
    ///
    /// These are the names/identifiers for different variables
    /// List index 1
    pub var_keywords: [&'a str; 5],
    /// Config file keys
    pub config_keys: [&'a str; 3],
    /// Manifest file keys
    pub manifest_keys: [&'a str; 7],
    /// Flavour names. This will robably be changed later
    pub flavours: [&'a str; 2]
}

impl Keywords<'_> {
    /// Checks is an identifier is either a keyword or variable keyword
    ///
    /// Returns the list index, and index within the list
    pub fn contains(&self, identifier: &str) -> Option<(u8, u16)> {
        match self.keywords.iter().position(|&x| x == identifier) {
            Some(position) => return Some((0, position as u16)),
            _ => {}
        }
        match self.var_keywords.iter().position(|&x| x == identifier) {
            Some(position) => return Some((1, position as u16)),
            _ => {}
        }
        None
    }
}

/// Messages for different events
///
/// this is mostly a holder to contain different message structs
pub struct Messages<'a> {
    /// Generic messages. Currently only holding a message about the shell language changing
    pub generic: [&'a str; 1],
    /// Error and warning related messages
    pub errors: ErrorHolder<'a>
}

/// Error and warning related messages
///
/// Contains all the errors that can be returned
pub struct ErrorHolder<'a> {
    /// Errors related to the language part of fck
    /// Code 00**
    pub language_errors: [ErrorMessages<'a>; 2],
    /// Unknown item errors
    /// Code 01**
    pub unknown_errors: [ErrorMessages<'a>; 2],
    /// Expectation errors relating to the parser expecting something that wasn't there
    /// Code 02**
    pub expected_errors: [ErrorMessages<'a>; 9],
    /// Errors relating to the use of something where it can't be
    /// Currently only has an error when a variable name is given as a keyword
    /// Code 03**
    pub not_here_errors: [ErrorMessages<'a>; 1],
    /// Type related errors, such as returning the wrong type
    /// Code 04**
    pub type_errors: [ErrorMessages<'a>; 2],
}

impl ErrorHolder<'_> {
    /// Gets the name of the error section
    fn get_name(&self, code: u16) -> &'_ str {
        let index = (code / 100)  as usize;
        match code % 100u16 {
            1u16 => self.language_errors.get(index).unwrap(),
            2u16 => self.unknown_errors.get(index).unwrap(),
            3u16 => self.expected_errors.get(index).unwrap(),
            _ => unreachable!()
        }.name
    }
}

/// Error message struct
///
/// Holds the error name and description
pub struct ErrorMessages<'a> {
    /// Error name
    pub name: &'a str,
    /// Error description
    pub desc: &'a str,
    // pub long_desc: String
}

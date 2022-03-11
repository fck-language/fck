#[derive(Clone, Copy)]
pub struct Keywords<'a> {
    pub keywords: [&'a str; 22],
    pub var_keywords: [&'a str; 5],
    pub config_keys: [&'a str; 3],
    pub manifest_keys: [&'a str; 7],
    pub flavours: [&'a str; 2]
}

impl Keywords<'_> {
    pub fn contains(&self, identifier: &str) -> Option<(u8, u8)> {
        match self.keywords.iter().position(|&x| x == identifier) {
            Some(position) => return Some((0, position as u8)),
            _ => {}
        }
        match self.var_keywords.iter().position(|&x| x == identifier) {
            Some(position) => return Some((1, position as u8)),
            _ => {}
        }
        None
    }
}

pub struct Messages<'a> {
    pub generic: [&'a str; 1],
    pub errors: ErrorHolder<'a>
}

pub struct ErrorHolder<'a> {
    pub language_errors: [ErrorMessages<'a>; 2],
    pub unknown_errors: [ErrorMessages<'a>; 2],
    pub expected_errors: [ErrorMessages<'a>; 9],
    pub not_here_errors: [ErrorMessages<'a>; 1],
    pub type_errors: [ErrorMessages<'a>; 1],
}

impl ErrorHolder<'_> {
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

pub struct ErrorMessages<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    // pub long_desc: String
}

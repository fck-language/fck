#[derive(Clone, Copy)]
pub struct Keywords<'a> {
    pub keywords: [&'a str; 20],
    pub var_keywords: [&'a str; 5]
}

impl Keywords<'_> {
    pub fn contains(&self, identifier: &str) -> Option<String> {
        match self.keywords.iter().position(|&x| x == identifier) {
            Some(position) => return Some(format!("0.{}", position)),
            _ => {}
        }
        match self.var_keywords.iter().position(|&x| x == identifier) {
            Some(position) => return Some(format!("1.{}", position)),
            _ => {}
        }
        None
    }
}

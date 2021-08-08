pub struct Keywords<'a> {
    pub keywords: [&'a str; 21],
    pub var_keywords: [&'a str; 5]
}

impl Keywords<'_> {
    pub fn contains(&self, identifier: &str) -> bool {
        let mut out = self.keywords.contains(&identifier);
        if out {
            return out;
        }
        out = self.var_keywords.contains(&identifier);
        return out;
    }
}

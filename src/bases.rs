#[derive(Copy, Clone)]
pub struct Position {
    ln: usize,
    pub col: usize
}

impl Position {
    pub fn new() -> Position {
        return Position{ln: 0, col: 0}
    }
    pub fn advance(&mut self) {
        self.col += 1
    }
    pub fn advance_ln(&mut self) {
        self.ln += 1;
        self.col = 0
    }
    pub fn generate_position(&self) -> Position {
        return Position{ln: self.ln, col: self.col}
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[ln:{:02}, col:{:02}]", self.ln, self.col)
    }
}

#[derive(Clone)]
pub struct Token {
    pub type_: u8,
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position
}

impl Token {
    pub fn new(type_: u8, value: String, pos_start: Position, pos_end: Position) -> Token {
        return Token{type_, value, pos_start, pos_end};
    }
    pub fn blank() -> Token {
        return Token::new(0, "".into(), Position::new(), Position::new());
    }
    pub fn matches(&self, type_: u8, value: &str) -> bool {
        return self.type_ == type_ && self.value == String::from(value);
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Type: {:<3} (Value: {})\n\tpos_start: {}\n\tpos_end  : {}",
               self.type_, self.value, self.pos_start, self.pos_end)
    }
}

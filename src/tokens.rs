pub const TT_INT: u8 =               0;
pub const TT_FLOAT: u8 =             1;
pub const TT_STRING: u8 =            2;
pub const TT_PLUS: u8 =              3;
pub const TT_MINUS: u8 =             4;
pub const TT_MOD: u8 =               5;
pub const TT_MULT: u8 =              6;
pub const TT_DIV: u8 =               7;
pub const TT_FDIV: u8 =              8;
pub const TT_POW: u8 =               9;
pub const TT_LPAREN: u8 =            10;
pub const TT_RPAREN: u8 =            11;
pub const TT_LPAREN_CURLY: u8 =      12;
pub const TT_RPAREN_CURLY: u8 =      13;
pub const TT_LPAREN_SQUARE: u8 =     14;
pub const TT_RPAREN_SQUARE: u8 =     15;
pub const TT_EOF: u8 =               16;
pub const TT_AT: u8 =                17;
pub const TT_NOT: u8 =               18;
pub const TT_COLON: u8 =             19;
pub const TT_IDENTIFIER: u8 =        20;
pub const TT_KEYWORD: u8 =           21;
pub const TT_QUESTION_MARK: u8 =     22;
pub const TT_DOT: u8 =               23;
pub const TT_EQ: u8 =                24;
pub const TT_NE: u8 =                25;
pub const TT_LT: u8 =                26;
pub const TT_GT: u8 =                27;
pub const TT_LTE: u8 =               28;
pub const TT_GTE: u8 =               29;
pub const TT_COMMA: u8 =             30;
pub const TT_NEWLINE: u8 =           31;
// Just set equivalents are just the number -8
// To check if the token is a SET, it has to be in the range 32 <= x <= 39
// SET_RET is in the range 40 <= x <= 47
// ALL variable SET or SET_RET tokens are 32 <= x <= 47
// To calculate the type of operator, just get the token of the nested operator
// and add it to either SET or SET_RET
pub const TT_SET: u8 =               32;
pub const TT_SET_PLUS: u8 =          33;
pub const TT_SET_MINUS: u8 =         34;
pub const TT_SET_MOD: u8 =           35;
pub const TT_SET_MULT: u8 =          36;
pub const TT_SET_DIV: u8 =           37;
pub const TT_SET_FDIV: u8 =          38;
pub const TT_SET_POW: u8 =           39;
pub const TT_SET_RET: u8 =           40;
pub const TT_SET_RET_PLUS: u8 =      41;
pub const TT_SET_RET_MINUS: u8 =     42;
pub const TT_SET_RET_MOD: u8 =       43;
pub const TT_SET_RET_MULT: u8 =      44;
pub const TT_SET_RET_DIV: u8 =       45;
pub const TT_SET_RET_FDIV: u8 =      46;
pub const TT_SET_RET_POW: u8 =       47;

use fck;
use fck::{
	ast::{ Lexer, Parser },
};

#[cfg(test)]
mod no_nesting {
	use fck::ast::{ Lexer, Parser };
	use lang::en::KEYWORDS;
	
	#[test]
	fn if_bool() {
		let toks = Lexer::new("if true\n\tint a = 1\n}".to_string(), KEYWORDS, "en".to_string()).make_tokens().unwrap();
		assert!(Parser::new(toks).parse().is_ok());
	}
}

use fck;
use fck::{
	ast::{Lexer, Parser}
};
use lang::en::KEYWORDS;

#[test]
fn blank_entry() {
	let toks = fck::ast::Lexer::new(String::new(), KEYWORDS, String::from("en")).make_tokens().unwrap();
	assert_eq!(toks.len(), 0);
}

#[cfg(test)]
mod brackets {
	use lang::en::KEYWORDS;
	use fck::ast::{Lexer, Parser};
	
	#[test]
	fn unmatched_curly_brackets() {
		let toks = Lexer::new(String::from("if true {\n\tint a :: -5\n}}"), KEYWORDS, String::from("en")).make_tokens().unwrap();
		assert!(Parser::new(toks).parse().is_err());
	}
}

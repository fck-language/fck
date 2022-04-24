use fck;
use fck::{
	ast::{ Lexer, Parser },
};

#[cfg(test)]
mod no_nesting_if {
	#[cfg(test)]
	mod if_only {
		use fck::ast::{ Lexer, Parser };
		use lang::en::KEYWORDS;
		
		#[test]
		fn no_newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if 5 < 9 {\n\tint a = 1\n}".to_string(),
						KEYWORDS, "en".to_string()
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
		
		#[test]
		fn newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if true\n{\n\tint a = 1\n}".to_string(),
						KEYWORDS, "en".to_string()
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
	}
	
	#[cfg(test)]
	mod if_elif {
		use fck::ast::{ Lexer, Parser };
		use lang::en::KEYWORDS;
		
		#[test]
		fn no_newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if true\n{\n\t12 * 9 - 3\n}\nelif 9 < 12 > 0\n{\n\t\"eeeee\"\n}".to_string(),
						KEYWORDS, "en".to_string()
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
		
		#[test]
		fn newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if true {\n\t12 * 9 - 3\n} elif 9 < 12 > 0 {\n\t\"eeeee\"\n}".to_string(),
						KEYWORDS, "en".to_string()
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
	}
	
	#[cfg(test)]
	mod if_else {
		use fck::ast::{ Lexer, Parser };
		use lang::en::KEYWORDS;
		
		#[test]
		fn no_newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if 9 > 100 {\n\t12 * 9 - 3\n} else {\n\t-5 + 2\n}".to_string(),
						KEYWORDS, "en".to_string()
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
		
		#[test]
		fn newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if 9 > 100\n{\n\t12 * 9 - 3\n}\nelse\n{\n\t-5 + 2\n}".to_string(),
						KEYWORDS, "en".to_string()
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
	}
	
	#[cfg(test)]
	mod if_elif_else {
		use fck::ast::{ Lexer, Parser };
		use lang::en::KEYWORDS;
		
		#[test]
		fn no_newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if false\n{\n\t12 * 9 - 3\n}\nelif -4 > 3\n{\n\t\"eeeee\"\n}\nelse\n{\n\t5\n}".to_string(),
						KEYWORDS, "en".to_string(),
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
		
		#[test]
		fn newline() {
			assert!(
				Parser::new(
					Lexer::new(
						"if false {\n\t12 * 9 - 3\n} elif -4 > 3 {\n\t\"eeeee\"\n} else {\n\t5\n}".to_string(),
						KEYWORDS, "en".to_string()
					).make_tokens().unwrap()
				).parse().is_ok()
			)
		}
	}
}

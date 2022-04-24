use fck;

#[cfg(test)]
mod full_line_comment {
	use fck::ast::Lexer;
	use lang::en::KEYWORDS;
	
	#[test]
	fn in_root() {
		assert!(Lexer::new(String::from("## this is a comment\nint a"), KEYWORDS, String::from("en")).make_tokens().is_ok())
	}
	
	#[test]
	fn after_something() {
		assert!(Lexer::new(String::from("int a :: 5 + 9 * 3 ## this is a comment"), KEYWORDS, String::from("en")).make_tokens().is_ok())
	}
	
	#[cfg(test)]
	mod in_conditional {
		use fck::ast::Lexer;
		use lang::en::KEYWORDS;
		
		#[test]
		fn with_indent() {
			assert!(Lexer::new(String::from("if false {\n\t## this is a comment\n\tint a\n}"), KEYWORDS, String::from("en")).make_tokens().is_ok())
		}
		
		#[test]
		fn without_indent() {
			assert!(Lexer::new(String::from("if false {\n## this is a comment\n\tint a\n}"), KEYWORDS, String::from("en")).make_tokens().is_ok())
		}
	}
}

#[cfg(test)]
mod inline_comment {
	use fck::ast::Lexer;
	use lang::en::KEYWORDS;
	
	#[test]
	fn in_root() {
		assert!(Lexer::new(String::from("# this is a comment #\nint a :: 5"), KEYWORDS, String::from("en")).make_tokens().is_ok())
	}
	
	#[test]
	fn after_something() {
		assert!(Lexer::new(String::from("int a :: 5## this is a comment"), KEYWORDS, String::from("en")).make_tokens().is_ok())
	}
	
	#[test]
	fn in_th_middle_of_something() {
		assert!(Lexer::new(String::from("int a :: # this is a bad comment placement # 5"), KEYWORDS, String::from("en")).make_tokens().is_ok())
	}
	
	#[cfg(test)]
	mod in_conditional {
		use fck::ast::Lexer;
		use lang::en::KEYWORDS;
		
		#[test]
		fn with_indent() {
			assert!(Lexer::new(String::from("if false {\n\t# this is a comment #int a\n}"), KEYWORDS, String::from("en")).make_tokens().is_ok())
		}
		
		#[test]
		fn without_indent() {
			assert!(Lexer::new(String::from("if false {\n# this is a comment #int a\n}"), KEYWORDS, String::from("en")).make_tokens().is_ok())
		}
	}
}

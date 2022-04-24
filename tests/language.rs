use fck;
use fck::{
	ast::Lexer,
	bases::{ Token, TokType }
};
use lang::*;

#[test]
fn de() {
	let toks_en = Lexer::new(en::KEYWORDS.keywords.join("\n"), en::KEYWORDS, "en".to_string()).make_tokens().unwrap();
	let toks_de = Lexer::new(de::KEYWORDS.keywords.join("\n"), de::KEYWORDS, "de".to_string()).make_tokens().unwrap();
	toks_en.iter().zip(toks_de.iter()).map(
		|(t_en, t_de)| {
			assert_eq!(t_en.type_, t_de.type_);
			assert_eq!(t_en.pos_start, t_de.pos_start);
			assert_eq!(t_en.pos_end.ln, t_de.pos_end.ln)
		}
	);
}

#[test]
fn fr() {
	let toks_en = Lexer::new(en::KEYWORDS.keywords.join("\n"), en::KEYWORDS, "en".to_string()).make_tokens().unwrap();
	let toks_de = Lexer::new(fr::KEYWORDS.keywords.join("\n"), fr::KEYWORDS, "fr".to_string()).make_tokens().unwrap();
	toks_en.iter().zip(toks_de.iter()).map(
		|(t_en, t_de)| {
			assert_eq!(t_en.type_, t_de.type_);
			assert_eq!(t_en.pos_start, t_de.pos_start);
			assert_eq!(t_en.pos_end.ln, t_de.pos_end.ln)
		}
	);
}

#[test]
fn ko() {
	let toks_en = Lexer::new(en::KEYWORDS.keywords.join("\n"), en::KEYWORDS, "en".to_string()).make_tokens().unwrap();
	let toks_de = Lexer::new(ko::KEYWORDS.keywords.join("\n"), ko::KEYWORDS, "ko".to_string()).make_tokens().unwrap();
	toks_en.iter().zip(toks_de.iter()).map(
		|(t_en, t_de)| {
			assert_eq!(t_en.type_, t_de.type_);
			assert_eq!(t_en.pos_start, t_de.pos_start);
			assert_eq!(t_en.pos_end.ln, t_de.pos_end.ln)
		}
	);
}

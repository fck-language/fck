use itertools::Itertools;
use fck;
use fck::{
	ast::Lexer,
	bases::{ Token, TokType }
};
use lang::*;

#[test]
fn language_equivalence() {
	for t in vec![
		(en::KEYWORDS, "en"),
		(de::KEYWORDS, "de"),
		(fr::KEYWORDS, "fr"),
		(ko::KEYWORDS, "ko")
	].iter().combinations(2) {
		let k0 = t.first().unwrap().0;
		let n0 = t.first().unwrap().1.to_string();
		let k1 = t.last().unwrap().0;
		let n1 = t.last().unwrap().1.to_string();
		let t0 = Lexer::new(k0.keywords.join("\n"), k0, n0).make_tokens().unwrap();
		let t1 = Lexer::new(k1.keywords.join("\n"), k1, n1).make_tokens().unwrap();
		t0.iter().zip(t1.iter()).map(
		|(t_0, t_1)| {
			assert_eq!(t_0.type_, t_1.type_);
			assert_eq!(t_0.pos_start, t_1.pos_start);
			assert_eq!(t_0.pos_end.ln, t_1.pos_end.ln)
		}
	);
	}
}

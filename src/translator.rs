use lang::{
	get_associated_keywords,
	keywords::Keywords
};
use reqwest::{ blocking::Client, header::CONTENT_TYPE };
use std::collections::HashMap;
use crate::{
	ast::Lexer,
	err_wrn::Error,
	bases::{ Token, Position, TokType }
};

pub fn translate(contents: String, initial_lang: Keywords<'static>, init_lang_code: String, target_lang_code: &str, mut comments: bool) -> Result<String, Error> {
	let target_keywords = match get_associated_keywords(target_lang_code) {
		Some(k) => k,
		None => {
			todo!("Make error here")
		}
	};
	let client = Client::new();
	let lex = match Lexer::new(contents, initial_lang, init_lang_code, true).make_tokens() {
		Ok(toks) => toks,
		Err(e) => return Err(e)
	};
	
	let mut res = String::new();
	let mut previous = Position::new();
	previous.advance();
	for t in lex {
		if t.pos_start.ln != previous.ln {
			if (t.pos_start.col - 1) % 4 == 0 {
				res.extend(vec!['\t'].repeat((t.pos_start.col - 1) / 4))
			} else {
				res.extend(vec![' '].repeat(t.pos_start.col - 1))
			}
		} else {
			res.extend(vec![' '].repeat(t.pos_start.col - previous.col))
		}
		res.extend(
			match t.type_ {
				TokType::Int(i) => format!("{}", i),
				TokType::Float(i) => format!("{}", i),
				TokType::String(i) => format!("{:?}", i),
				TokType::Plus => "+".to_string(),
				TokType::Minus => "-".to_string(),
				TokType::Mod => "%".to_string(),
				TokType::Mult => "*".to_string(),
				TokType::Div => "/".to_string(),
				TokType::FDiv => "//".to_string(),
				TokType::Pow => "**".to_string(),
				TokType::Increment => "++".to_string(),
				TokType::Decrement => "--".to_string(),
				TokType::LParen => "(".to_string(),
				TokType::RParen => ")".to_string(),
				TokType::LParenCurly => "{".to_string(),
				TokType::RParenCurly => "}".to_string(),
				TokType::LParenSquare => "[".to_string(),
				TokType::RParenSquare => "]".to_string(),
				TokType::Label(l) => format!("@{}", l),
				TokType::Not => "!".to_string(),
				TokType::Colon => ":".to_string(),
				TokType::Identifier(i, n) => format!("{}:{}", i, n),
				TokType::Keyword(i, n) => target_keywords.get_word(i, n).to_string(),
				TokType::QuestionMark => "?".to_string(),
				TokType::Dot => ".".to_string(),
				TokType::Eq => "==".to_string(),
				TokType::NE => "!=".to_string(),
				TokType::LT => "<".to_string(),
				TokType::GT => ">".to_string(),
				TokType::LTE => "<=".to_string(),
				TokType::GTE => ">=".to_string(),
				TokType::Comma => ",".to_string(),
				TokType::Newline => {
					if t.pos_start.col == t.pos_end.col { "\n" } else { ";" }.to_string()
				},
				TokType::Set => "=".to_string(),
				TokType::SetPlus => "+=".to_string(),
				TokType::SetMinus => "-=".to_string(),
				TokType::SetMod => "%=".to_string(),
				TokType::SetMult => "*=".to_string(),
				TokType::SetDiv => "/=".to_string(),
				TokType::SetFDiv => "//=".to_string(),
				TokType::SetPow => "**=".to_string(),
				TokType::Comment(c, l) => {
					let mut inner = c;
					if comments {
						let pre = inner.chars().map_while(|c| if c == ' ' { Some(' ') } else { None }).collect::<Vec<_>>().len();
						let post = inner.chars().rev().map_while(|c| if c == ' ' { Some(' ') } else { None }).collect::<Vec<_>>().len();
						inner = inner.get(pre..inner.len() - post).unwrap().to_string();
						match client
							.post("https://libretranslate.de/translate")
							.body(format!("{{\"q\":\"{}\",\"source\":\"{}\",\"target\":\"{}\",\"format\":\"text\"}}", inner, l, target_lang_code))
							.header(CONTENT_TYPE, "application/json").send() {
							Ok(r) => {
								if r.status().as_u16() == 200 {
									let temp = r.text().unwrap();
									let temp = temp.get(19..temp.len() - 3).unwrap().to_string();
									
									// find unicode characters and replace
									let mut split = temp.split("\\u").collect::<Vec<&str>>();
									let mut out = split.first().unwrap().to_string();
									split.remove(0);
									for t in split {
										out.extend(match u32::from_str_radix(&t[0..4], 16) {
											Ok(n) => format!("{}{}", std::char::from_u32(n).unwrap(), &t[4..]),
											Err(_) => format!("\\u{}", t)
										}.chars());
									}
									inner = out;
								} else {
									comments = false;
									println!("Got {}", r.status());
									break
								}
							},
							Err(e) => {
								println!("Got {}", e.status().unwrap());
								break
							}
						};
						inner = format!("{}{}{}", " ".repeat(pre), inner, " ".repeat(post));
					}
					if t.pos_end.ln != t.pos_start.ln { format!("##{}\n", inner) } else { format!("#{}#", inner) }
				}
			}.chars()
		);
		previous = t.pos_end.clone()
	}
	Ok(res)
}
use lang::{
	get_associated_keywords,
	keywords::Keywords
};

pub fn translate(file_name: &str, mut initial_lang: Keywords, target_lang_code: &str) -> Result<(), String> {
	if !std::path::Path::new(file_name).exists() {
		return Err(format!("File \"{}\" does not exist", file_name))
	}
	let mut translated = format!("#!{}", target_lang_code);
	let target_lang = match get_associated_keywords(target_lang_code) {
		Some(k) => k,
		None => return Err(format!("Unknown lang code \"{}\"", target_lang_code))
	};
	let original = match std::fs::read_to_string(file_name) {
		Ok(s) => s.chars().collect::<Vec<char>>(),
		Err(e) => return Err(format!("{}", e))
	};
	println!("{}", std::fs::read_to_string(file_name).unwrap());
	let mut iter = original.iter();
	let mut blank = 0usize;
	loop {
		if let Some(c) = iter.next() {
			let mut temp = String::new();
			if c == &'#' {
				if let Some(c) = iter.next() {
					if c == &'#' {
						while let Some(c) = iter.next() {
							if c == &'\n' { break }
						}
					} else if c == &'!' {
						let mut new_code = String::new();
						if let Some(c) = iter.next() {
							new_code.push(*c)
						} else {
							translated.push_str("#!");
							break
						}
						if let Some(c) = iter.next() {
							new_code.push(*c)
						} else {
							translated.push_str("#!");
							translated.push_str(&*new_code);
							break
						}
						initial_lang = get_associated_keywords(&*new_code)
							.unwrap_or(initial_lang);
						if let Some(c) = iter.next() {
							if c != &'\n' {
								translated.push(*c)
							}
						} else {
							break
						}
					}
				}
			} else if c.is_alphabetic() {
				temp.push(*c);
				while let Some(c) = iter.next() {
					if c.is_alphabetic() {
						temp.push(*c);
					} else {
						// Get keyword index
						if let Some(i) = initial_lang.keywords.iter().position(|&arg| arg == &*temp) {
							translated.push_str(target_lang.keywords.get(i).unwrap())
						} else if let Some(i) = initial_lang.var_keywords.iter().position(|&arg| arg == &*temp) {
							translated.push_str(target_lang.var_keywords.get(i).unwrap())
						} else {
							translated.push_str(&*temp);
						}
						translated.push(*c);
						break
					}
				}
			} else if c == &' ' || c == &'\t' {
				blank += 1;
			} else {
				translated.push(*c);
			}
		} else {
			break
		}
	}
	println!("**\n{}\n**", translated);
	if let Err(e) = std::fs::write(
		format!("{}_{}.fck",
				file_name.get(..file_name.len() - 4).unwrap(),
				target_lang_code
		), translated) {
		Err(format!("{}", e))
	} else {
		Ok(())
	}
}
use std::fmt::Write;

pub struct Json(pub String);

impl Json {
	pub fn new() -> Self {
		Self(String::with_capacity(4096))
	}

	pub fn raw(&mut self, text: &str) -> &mut Self {
		self.0.push_str(text);
		self
	}

	pub fn string(&mut self, text: &str) -> &mut Self {
		self.0.push('"');
		for character in text.chars() {
			match character {
				'"' => self.0.push_str("\\\""),
				'\\' => self.0.push_str("\\\\"),
				'\n' => self.0.push_str("\\n"),
				'\r' => self.0.push_str("\\r"),
				'\t' => self.0.push_str("\\t"),
				control if u32::from(control) < 0x20 => {
					let _ = write!(self.0, "\\u{:04x}", u32::from(control));
				}
				other => self.0.push(other),
			}
		}
		self.0.push('"');
		self
	}

	pub fn number(&mut self, value: impl std::fmt::Display) -> &mut Self {
		let _ = write!(self.0, "{value}");
		self
	}

	pub fn separator(&mut self, first: bool) -> &mut Self {
		if !first {
			self.0.push(',');
		}
		self
	}
}

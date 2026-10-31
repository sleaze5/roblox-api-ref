use std::collections::HashMap;

use full_moon::tokenizer::TokenReference;

use crate::api::code::CodeTone;

#[derive(Default)]
pub struct Roles(HashMap<usize, CodeTone>);

impl Roles {
	pub fn mark(&mut self, token: &TokenReference, tone: CodeTone) {
		self.0.insert(token.token().start_position().bytes(), tone);
	}

	pub fn take(&mut self, position: usize) -> Option<CodeTone> {
		self.0.remove(&position)
	}
}

use std::collections::HashMap;

use full_moon::tokenizer::TokenReference;

use crate::api::code::CodeTone;
use crate::model::DocLink;

#[derive(Clone, Debug)]
pub struct Role {
	pub tone: CodeTone,
	pub link: Option<DocLink>,
}

#[derive(Default)]
pub struct Roles(HashMap<usize, Role>);

pub fn position(token: &TokenReference) -> usize {
	token.token().start_position().bytes()
}

impl Roles {
	pub fn mark(&mut self, token: &TokenReference, tone: CodeTone, link: Option<DocLink>) {
		self.0.insert(position(token), Role { tone, link });
	}

	pub fn take(&mut self, position: usize) -> Option<Role> {
		self.0.remove(&position)
	}
}

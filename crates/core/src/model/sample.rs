use crate::api::code::CodeTone;

use super::DocLink;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampleToken {
	pub text: String,
	pub tone: CodeTone,
	pub link: Option<DocLink>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeSample {
	pub source: String,
	pub tokens: Vec<SampleToken>,
}

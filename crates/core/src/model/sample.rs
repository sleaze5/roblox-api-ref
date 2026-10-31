use crate::api::code::CodeTone;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampleToken {
	pub text: String,
	pub tone: CodeTone,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeSample {
	pub source: String,
	pub lines: Vec<Vec<SampleToken>>,
}

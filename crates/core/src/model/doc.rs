use rapi_routes::MemberKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocLink {
	Class {
		name: String,
	},
	Member {
		owner: String,
		member: MemberKind,
		name: String,
	},
	Enum {
		name: String,
	},
	EnumItem {
		owner: String,
		name: String,
	},
	Type {
		name: String,
	},
	External {
		url: String,
	},
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DocTag {
	A,
	Br,
	Code,
	Em,
	Kbd,
	Li,
	Pre,
	Strong,
	Sup,
	Ul,
}

impl DocTag {
	pub fn parse(name: &str) -> Option<Self> {
		Some(match name {
			"a" => Self::A,
			"br" => Self::Br,
			"code" => Self::Code,
			"em" => Self::Em,
			"kbd" => Self::Kbd,
			"li" => Self::Li,
			"pre" => Self::Pre,
			"strong" => Self::Strong,
			"sup" => Self::Sup,
			"ul" => Self::Ul,
			_ => return None,
		})
	}

	pub const fn name(self) -> &'static str {
		match self {
			Self::A => "a",
			Self::Br => "br",
			Self::Code => "code",
			Self::Em => "em",
			Self::Kbd => "kbd",
			Self::Li => "li",
			Self::Pre => "pre",
			Self::Strong => "strong",
			Self::Sup => "sup",
			Self::Ul => "ul",
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocElement {
	pub tag: DocTag,
	pub children: Vec<DocNode>,
	pub link: Option<DocLink>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocNode {
	Text(String),
	Element(DocElement),
}

pub type Doc = Vec<DocNode>;

pub fn doc_text(nodes: &[DocNode]) -> String {
	let mut text = String::new();
	push_text(&mut text, nodes);
	text
}

fn push_text(text: &mut String, nodes: &[DocNode]) {
	for node in nodes {
		match node {
			DocNode::Text(value) => text.push_str(value),
			DocNode::Element(element) => push_text(text, &element.children),
		}
	}
}

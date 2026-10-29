use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

use crate::js::JS_SPACE;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HtmlElement {
	pub name: String,
	pub attributes: HashMap<String, String>,
	pub children: Vec<HtmlNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HtmlNode {
	Text(String),
	Element(HtmlElement),
}

static TOKEN: LazyLock<Regex> = LazyLock::new(|| {
	let space = format!("[{JS_SPACE}]");
	let not_space = format!("[^{JS_SPACE}");
	let pattern = format!(
		r#"\A(?:<!--[\s\S]*?(?:-->|\z)|<(/?)([a-zA-Z][a-zA-Z0-9]*)((?:{space}+{not_space}"'>/=]+(?:{space}*={space}*(?:"[^"]*"|'[^']*'|{not_space}"'>]+))?)*){space}*(/?)>|([^<]+|<))"#
	);
	Regex::new(&pattern).expect("the token pattern compiles")
});

static ATTRIBUTE: LazyLock<Regex> = LazyLock::new(|| {
	let space = format!("[{JS_SPACE}]");
	let not_space = format!("[^{JS_SPACE}");
	let pattern = format!(
		r#"({not_space}"'>/=]+)(?:{space}*={space}*(?:"([^"]*)"|'([^']*)'|({not_space}"'>]+)))?"#
	);
	Regex::new(&pattern).expect("the attribute pattern compiles")
});

static ENTITY: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"&(?:#[xX]([0-9a-fA-F]+)|#([0-9]+)|([a-zA-Z][a-zA-Z0-9]*));")
		.expect("the entity pattern compiles")
});

fn named_entity(name: &str) -> Option<&'static str> {
	Some(match name {
		"amp" => "&",
		"lt" => "<",
		"gt" => ">",
		"quot" => "\"",
		"apos" => "'",
		"nbsp" => "\u{a0}",
		"lsquo" => "\u{2018}",
		"rsquo" => "\u{2019}",
		"ldquo" => "\u{201c}",
		"rdquo" => "\u{201d}",
		"mdash" => "\u{2014}",
		"ndash" => "\u{2013}",
		"times" => "\u{d7}",
		"deg" => "\u{b0}",
		"sup2" => "\u{b2}",
		"sup3" => "\u{b3}",
		"hellip" => "\u{2026}",
		"rarr" => "\u{2192}",
		"larr" => "\u{2190}",
		"le" => "\u{2264}",
		"ge" => "\u{2265}",
		"plusmn" => "\u{b1}",
		_ => return None,
	})
}

fn code_point(digits: &str, radix: u32) -> Option<char> {
	let trimmed = digits.trim_start_matches('0');
	if trimmed.len() > 8 {
		return None;
	}
	let value = u32::from_str_radix(if trimmed.is_empty() { "0" } else { trimmed }, radix).ok()?;
	if value == 0 || value > 0x10_ffff {
		return None;
	}
	Some(char::from_u32(value).unwrap_or('\u{fffd}'))
}

pub fn decode_entities(text: &str) -> String {
	if !text.contains('&') {
		return text.to_owned();
	}
	ENTITY
		.replace_all(text, |captures: &regex::Captures<'_>| {
			let decoded = if let Some(hex) = captures.get(1) {
				code_point(hex.as_str(), 16).map(String::from)
			} else if let Some(decimal) = captures.get(2) {
				code_point(decimal.as_str(), 10).map(String::from)
			} else {
				captures
					.get(3)
					.and_then(|name| named_entity(name.as_str()))
					.map(str::to_owned)
			};
			decoded.unwrap_or_else(|| captures[0].to_owned())
		})
		.into_owned()
}

fn parse_attributes(source: &str) -> HashMap<String, String> {
	ATTRIBUTE
		.captures_iter(source)
		.map(|captures| {
			let value = captures
				.get(2)
				.or_else(|| captures.get(3))
				.or_else(|| captures.get(4))
				.map_or("", |value| value.as_str());
			(captures[1].to_lowercase(), decode_entities(value))
		})
		.collect()
}

fn children_mut<'a>(root: &'a mut Vec<HtmlNode>, path: &[usize]) -> &'a mut Vec<HtmlNode> {
	let mut children = root;
	for &index in path {
		match &mut children[index] {
			HtmlNode::Element(element) => children = &mut element.children,
			HtmlNode::Text(_) => unreachable!("open elements are always elements"),
		}
	}
	children
}

pub fn parse_html(source: &str) -> Vec<HtmlNode> {
	let mut root = Vec::new();
	let mut open: Vec<(String, usize)> = Vec::new();
	let mut position = 0;
	while position < source.len() {
		let Some(captures) = TOKEN.captures(&source[position..]) else {
			break;
		};
		let whole = captures.get(0).map_or(0, |token| token.end());
		position += whole.max(1);
		let path: Vec<usize> = open.iter().map(|(_, index)| *index).collect();
		if let Some(text) = captures.get(6) {
			children_mut(&mut root, &path).push(HtmlNode::Text(decode_entities(text.as_str())));
		} else if let Some(raw_name) = captures.get(2) {
			let name = raw_name.as_str().to_lowercase();
			if captures.get(1).is_some_and(|slash| slash.as_str() == "/") {
				if let Some(index) = open.iter().rposition(|(open_name, _)| *open_name == name) {
					open.truncate(index);
				}
			} else {
				let attributes = parse_attributes(captures.get(3).map_or("", |value| value.as_str()));
				let self_closing = captures.get(4).is_some_and(|slash| slash.as_str() == "/");
				let children = children_mut(&mut root, &path);
				children.push(HtmlNode::Element(HtmlElement {
					name: name.clone(),
					attributes,
					children: Vec::new(),
				}));
				if !self_closing && !matches!(name.as_str(), "br" | "hr" | "img" | "wbr") {
					open.push((name, children.len() - 1));
				}
			}
		}
	}
	root
}

#[cfg(test)]
mod tests {
	use super::*;

	fn element(name: &str, attributes: &[(&str, &str)], children: Vec<HtmlNode>) -> HtmlNode {
		HtmlNode::Element(HtmlElement {
			name: name.to_owned(),
			attributes: attributes
				.iter()
				.map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
				.collect(),
			children,
		})
	}

	fn text(value: &str) -> HtmlNode {
		HtmlNode::Text(value.to_owned())
	}

	#[test]
	fn builds_elements_decodes_entities_and_keeps_void_elements_empty() {
		assert_eq!(
			parse_html(r#"A&mdash;<code class="language-lua">x &lt; 2</code><br />B<img src='/a.png'>"#),
			vec![
				text("A—"),
				element("code", &[("class", "language-lua")], vec![text("x < 2")]),
				element("br", &[], vec![]),
				text("B"),
				element("img", &[("src", "/a.png")], vec![]),
			]
		);
	}

	#[test]
	fn tolerates_stray_closing_tags_unclosed_elements_and_a_lone_angle_bracket() {
		assert_eq!(
			parse_html("a < b</em><strong>c"),
			vec![text("a "), text("<"), text(" b"), element("strong", &[], vec![text("c")])]
		);
	}

	#[test]
	fn closes_the_innermost_matching_element_but_never_the_root() {
		assert_eq!(
			parse_html("<em>a<strong>b</em>c"),
			vec![element(
				"em",
				&[],
				vec![text("a"), element("strong", &[], vec![text("b")])]
			), text("c")]
		);
		assert_eq!(parse_html("<!-- hi -->x<!-- open"), vec![text("x")]);
	}

	#[test]
	fn decodes_numeric_and_named_entities_and_keeps_unknown_ones() {
		assert_eq!(decode_entities("&#x41;&#66;&hellip;&bogus;&#0;&AMP;"), "AB…&bogus;&#0;&AMP;");
	}
}

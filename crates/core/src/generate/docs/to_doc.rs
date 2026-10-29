use super::html::{HtmlElement, HtmlNode};
use crate::js::{collapse_spaces, js_trim, js_trim_end, js_trim_start};
use crate::model::{Doc, DocElement, DocLink, DocNode, DocTag, doc_text};

pub trait LinkResolver {
	fn href(&self, href: &str) -> Option<DocLink>;
	fn code(&self, text: &str) -> Option<DocLink>;
}

#[derive(Clone, Copy)]
struct Context<'a> {
	links: &'a dyn LinkResolver,
	pre: bool,
	linked: bool,
}

fn append(nodes: &mut Vec<DocNode>, node: DocNode) {
	if let (DocNode::Text(text), Some(DocNode::Text(last))) = (&node, nodes.last_mut()) {
		last.push_str(text);
		return;
	}
	if !matches!(&node, DocNode::Text(text) if text.is_empty()) {
		nodes.push(node);
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Edge {
	Start,
	End,
}

fn trim_node(node: &DocNode, edge: Edge) -> DocNode {
	match node {
		DocNode::Text(text) => DocNode::Text(
			match edge {
				Edge::Start => js_trim_start(text),
				Edge::End => js_trim_end(text),
			}
			.to_owned(),
		),
		DocNode::Element(element) => DocNode::Element(DocElement {
			children: trim_edge(element.children.clone(), edge),
			..element.clone()
		}),
	}
}

fn is_blank(node: &DocNode) -> bool {
	match node {
		DocNode::Text(text) => text.is_empty(),
		DocNode::Element(element) => element.tag == DocTag::Br || element.children.is_empty(),
	}
}

fn trim_edge(mut nodes: Vec<DocNode>, edge: Edge) -> Vec<DocNode> {
	while !nodes.is_empty() {
		let index = if edge == Edge::Start { 0 } else { nodes.len() - 1 };
		let node = trim_node(&nodes[index], edge);
		if !is_blank(&node) {
			nodes[index] = node;
			break;
		}
		nodes.remove(index);
	}
	nodes
}

fn trim_edges(nodes: Vec<DocNode>) -> Vec<DocNode> {
	trim_edge(trim_edge(nodes, Edge::Start), Edge::End)
}

fn link_of(element: &HtmlElement, children: &[DocNode], context: Context<'_>) -> Option<DocLink> {
	match element.name.as_str() {
		"a" => element
			.attributes
			.get("href")
			.and_then(|href| context.links.href(href)),
		"code" if !context.linked && !context.pre => context.links.code(&doc_text(children)),
		_ => None,
	}
}

fn convert_element(element: &HtmlElement, context: Context<'_>) -> Vec<DocNode> {
	if matches!(element.name.as_str(), "img" | "script" | "style") {
		return Vec::new();
	}
	let inner = Context {
		pre: context.pre || element.name == "pre",
		linked: context.linked || element.name == "a",
		..context
	};
	let block = matches!(element.name.as_str(), "ul" | "li" | "pre");
	let children = convert_nodes(&element.children, inner, block);
	let Some(tag) = DocTag::parse(&element.name) else {
		return children;
	};
	let link = link_of(element, &children, context);
	if tag == DocTag::A && link.is_none() {
		return children;
	}
	vec![DocNode::Element(DocElement {
		tag,
		children,
		link,
	})]
}

fn convert_nodes(nodes: &[HtmlNode], context: Context<'_>, block: bool) -> Vec<DocNode> {
	let mut result = Vec::new();
	for node in nodes {
		match node {
			HtmlNode::Element(element) => {
				for converted in convert_element(element, context) {
					append(&mut result, converted);
				}
			}
			HtmlNode::Text(text) if context.pre => append(&mut result, DocNode::Text(text.clone())),
			HtmlNode::Text(text) => {
				if !block || !js_trim(text).is_empty() {
					append(&mut result, DocNode::Text(collapse_spaces(text)));
				}
			}
		}
	}
	if block { trim_edges(result) } else { result }
}

pub fn normalize_text(text: &str) -> String {
	let mut normalized = String::with_capacity(text.len());
	let mut characters = text.chars().peekable();
	while let Some(character) = characters.next() {
		match character {
			'\r' => {
				if characters.peek() == Some(&'\n') {
					characters.next();
				}
				normalized.push('\n');
			}
			'\u{2018}' | '\u{2019}' | '\u{201a}' | '\u{201b}' => normalized.push('\''),
			'\u{201c}' | '\u{201d}' | '\u{201e}' | '\u{201f}' => normalized.push('"'),
			other => normalized.push(other),
		}
	}
	let mut collapsed = String::with_capacity(normalized.len());
	let mut newlines = 0;
	for character in normalized.chars() {
		if character == '\n' {
			newlines += 1;
			if newlines <= 2 {
				collapsed.push('\n');
			}
		} else {
			newlines = 0;
			collapsed.push(character);
		}
	}
	collapsed
}

fn normalize_nodes(nodes: Vec<DocNode>) -> Vec<DocNode> {
	nodes
		.into_iter()
		.map(|node| match node {
			DocNode::Text(text) => DocNode::Text(normalize_text(&text)),
			DocNode::Element(element) => DocNode::Element(DocElement {
				children: normalize_nodes(element.children),
				..element
			}),
		})
		.collect()
}

pub fn to_doc(nodes: &[HtmlNode], links: &dyn LinkResolver) -> Option<Doc> {
	let context = Context {
		links,
		pre: false,
		linked: false,
	};
	let doc = trim_edges(normalize_nodes(convert_nodes(nodes, context, false)));
	if js_trim(&doc_text(&doc)).is_empty() {
		None
	} else {
		Some(doc)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::generate::docs::html::parse_html;

	struct TestLinks;

	impl LinkResolver for TestLinks {
		fn href(&self, href: &str) -> Option<DocLink> {
			(!href.starts_with("javascript:")).then(|| DocLink::External {
				url: href.to_owned(),
			})
		}

		fn code(&self, text: &str) -> Option<DocLink> {
			(text == "Part").then(|| DocLink::Class {
				name: "Part".to_owned(),
			})
		}
	}

	fn convert(html: &str) -> Option<Doc> {
		to_doc(&parse_html(html), &TestLinks)
	}

	fn text(value: &str) -> DocNode {
		DocNode::Text(value.to_owned())
	}

	fn element(tag: DocTag, children: Vec<DocNode>, link: Option<DocLink>) -> DocNode {
		DocNode::Element(DocElement {
			tag,
			children,
			link,
		})
	}

	fn part() -> Option<DocLink> {
		Some(DocLink::Class {
			name: "Part".to_owned(),
		})
	}

	#[test]
	fn collapses_whitespace_trims_the_edges_and_removes_images() {
		assert_eq!(
			convert("  Shows   a <strong>part</strong>.  <img src=\"/x.png\" /> "),
			Some(vec![
				text("Shows a "),
				element(DocTag::Strong, vec![text("part")], None),
				text("."),
			])
		);
	}

	#[test]
	fn links_code_spans_but_not_code_inside_a_link_or_a_preformatted_block() {
		assert_eq!(
			convert(
				"<code>Part</code> <a href=\"https://x.dev\"><code>Part</code></a> <pre><code>Part</code></pre>"
			),
			Some(vec![
				element(DocTag::Code, vec![text("Part")], part()),
				text(" "),
				element(
					DocTag::A,
					vec![element(DocTag::Code, vec![text("Part")], None)],
					Some(DocLink::External {
						url: "https://x.dev".to_owned()
					})
				),
				text(" "),
				element(
					DocTag::Pre,
					vec![element(DocTag::Code, vec![text("Part")], None)],
					None
				),
			])
		);
	}

	#[test]
	fn unwraps_unknown_elements_and_links_without_a_target() {
		assert_eq!(
			convert("<span>a</span> <a href=\"javascript:x\">b</a>"),
			Some(vec![text("a b")])
		);
	}

	#[test]
	fn drops_whitespace_between_list_items() {
		assert_eq!(
			convert("<ul> <li> One </li> <li>Two</li> </ul>"),
			Some(vec![element(
				DocTag::Ul,
				vec![
					element(DocTag::Li, vec![text("One")], None),
					element(DocTag::Li, vec![text("Two")], None),
				],
				None
			)])
		);
	}

	#[test]
	fn normalizes_quotes_and_line_breaks_across_merged_text() {
		assert_eq!(
			convert("Says &ldquo;hi&rdquo; <pre>a\r\n\r\n<span>\r\n</span>\r\nb</pre>"),
			Some(vec![
				text("Says \"hi\" "),
				element(DocTag::Pre, vec![text("a\n\nb")], None),
			])
		);
	}

	#[test]
	fn trims_whitespace_line_breaks_and_empty_elements_at_block_edges() {
		assert_eq!(
			convert(
				"<br /> <strong> </strong>Hi <code>x </code><br /> <pre><code> y </code></pre><br />"
			),
			Some(vec![
				text("Hi "),
				element(DocTag::Code, vec![text("x ")], None),
				element(DocTag::Br, vec![], None),
				text(" "),
				element(
					DocTag::Pre,
					vec![element(DocTag::Code, vec![text("y")], None)],
					None
				),
			])
		);
	}

	#[test]
	fn returns_nothing_for_empty_documentation() {
		assert_eq!(convert("  <img src=\"/x.png\" /> "), None);
	}

	#[test]
	fn normalizes_text() {
		assert_eq!(normalize_text("“It’s ‘on’” „low‟"), "\"It's 'on'\" \"low\"");
		assert_eq!(normalize_text("a\r\nb\rc\n\n\n\nd\r\n\r\n\r\ne"), "a\nb\nc\n\nd\n\ne");
		assert_eq!(normalize_text("x — y → z"), "x — y → z");
	}
}

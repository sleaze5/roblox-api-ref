//! Fills each filter's match count, so the count shows before scripts run.

/// Counts the places where ` {name}` appears as a whole attribute name.
fn count_attribute(html: &str, name: &str) -> usize {
	let needle = format!(" {name}");
	html.match_indices(&needle)
		.filter(|(index, _)| {
			matches!(
				html.as_bytes().get(index + needle.len()),
				Some(b'=' | b'>' | b' ' | b'\t' | b'\n' | b'\r' | b'\x0c')
			)
		})
		.count()
}

/// The value of attribute `name` in an opening tag.
fn attribute<'a>(open: &'a str, name: &str) -> Option<&'a str> {
	let needle = format!(" {name}=\"");
	let start = open.find(&needle)? + needle.len();
	let end = open[start..].find('"')?;
	Some(&open[start..start + end])
}

fn has_attribute(open: &str, name: &str) -> bool {
	count_attribute(open, name) > 0
}

/// The element with `id`, from its opening tag up to its closing tag.
fn element_by_id<'a>(html: &'a str, id: &str) -> Option<&'a str> {
	let needle = format!(" id=\"{id}\"");
	let at = html.find(&needle)?;
	let start = html[..at].rfind('<')?;
	let open_end = start + html[start..].find('>')? + 1;
	let tag: String = html[start + 1..]
		.chars()
		.take_while(char::is_ascii_lowercase)
		.collect();
	let open_tag = format!("<{tag}");
	let close_tag = format!("</{tag}>");
	let mut depth = 1;
	let mut cursor = open_end;
	loop {
		let next_open = html[cursor..].find(&open_tag).map(|index| cursor + index);
		let next_close = html[cursor..]
			.find(&close_tag)
			.map(|index| cursor + index)?;
		match next_open {
			Some(open) if open < next_close => {
				let after = html.as_bytes().get(open + open_tag.len());
				if matches!(after, Some(b'>' | b' ' | b'\t' | b'\n' | b'\r' | b'\x0c')) {
					depth += 1;
				}
				cursor = open + open_tag.len();
			}
			_ => {
				depth -= 1;
				if depth == 0 {
					return Some(&html[start..next_close]);
				}
				cursor = next_close + close_tag.len();
			}
		}
	}
}

fn list_total(list: &str) -> usize {
	let open = &list[..list.find('>').map_or(list.len(), |end| end + 1)];
	if has_attribute(open, "data-tree") {
		list.match_indices("<li")
			.filter(|(index, _)| {
				matches!(
					list.as_bytes().get(index + 3),
					Some(b'>' | b' ' | b'\t' | b'\n' | b'\r' | b'\x0c')
				)
			})
			.count()
	} else if has_attribute(open, "data-changes") {
		count_attribute(list, "data-change")
	} else {
		list.matches(" data-name=\"").count()
	}
}

pub fn summary_text(shown: usize, total: usize) -> String {
	if shown == total {
		format!("{total} total")
	} else {
		format!("{shown} of {total}")
	}
}

/// Puts `text` into the first empty summary output of a form.
fn fill_summary(form: &str, text: &str) -> String {
	let mut cursor = 0;
	while let Some(found) = form[cursor..].find("<output") {
		let start = cursor + found;
		let Some(end) = form[start..].find('>').map(|end| start + end + 1) else {
			break;
		};
		if has_attribute(&form[start..end], "data-filter-summary")
			&& form[end..].starts_with("</output>")
		{
			return format!("{}{text}{}", &form[..end], &form[end..]);
		}
		cursor = end;
	}
	form.to_owned()
}

fn replace_forms(
	html: &str,
	matches: impl Fn(&str) -> bool,
	text: impl Fn(&str) -> Option<String>,
) -> String {
	let mut result = String::with_capacity(html.len() + 64);
	let mut last = 0;
	let mut cursor = 0;
	while let Some(found) = html[cursor..].find("<form") {
		let start = cursor + found;
		let Some(open_end) = html[start..].find('>').map(|end| start + end + 1) else {
			break;
		};
		let open = &html[start..open_end];
		cursor = open_end;
		if start < last || !matches(open) {
			continue;
		}
		let end = html[start..]
			.find("</form>")
			.map_or(html.len(), |end| start + end);
		result.push_str(&html[last..start]);
		match text(open) {
			Some(summary) => result.push_str(&fill_summary(&html[start..end], &summary)),
			None => result.push_str(&html[start..end]),
		}
		last = end;
	}
	result.push_str(&html[last..]);
	result
}

pub fn fill_filter_summaries(html: &str) -> String {
	let with_lists = replace_forms(
		html,
		|open| attribute(open, "data-list-filter").is_some_and(|target| !target.is_empty()),
		|open| {
			let list = element_by_id(html, attribute(open, "data-list-filter")?)?;
			let total = list_total(list);
			Some(attribute(open, "data-noun").map_or_else(
				|| summary_text(total, total),
				|noun| format!("{total} {noun}"),
			))
		},
	);
	let members = count_attribute(&with_lists, "data-member");
	replace_forms(
		&with_lists,
		|open| has_attribute(open, "data-member-filter"),
		|_| Some(summary_text(members, members)),
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	const FIELD: &str = r#"<output data-filter-summary="" aria-live="polite"></output>"#;

	#[test]
	fn counts_the_rows_of_a_list() {
		let html = format!(
			r#"<form data-list-filter="list" data-param="q">{FIELD}</form><ul id="list"><li data-name="A"></li><li data-name="B"><ul><li>x</li></ul></li></ul><ul><li data-name="C"></li></ul>"#
		);
		assert!(fill_filter_summaries(&html).contains(">2 total</output>"));
	}

	#[test]
	fn counts_every_node_of_a_tree_and_every_line_of_a_change_list() {
		let tree = format!(
			r#"<form data-list-filter="tree">{FIELD}</form><div id="tree" data-tree=""><ul><li><ul><li></li></ul></li></ul></div>"#
		);
		assert!(fill_filter_summaries(&tree).contains(">2 total</output>"));
		let changes = format!(
			r#"<form data-list-filter="changes" data-noun="changes">{FIELD}</form><div id="changes" data-changes=""><div data-change="">a</div><div data-change="" data-name="b"></div></div>"#
		);
		assert!(fill_filter_summaries(&changes).contains(">2 changes</output>"));
	}

	#[test]
	fn counts_the_members_on_a_class_page() {
		let html = format!(
			r#"<form data-member-filter="" role="search">{FIELD}</form><div data-member="" data-name="A"></div><div data-member="" data-name="B"></div>"#
		);
		assert!(fill_filter_summaries(&html).contains(">2 total</output>"));
	}

	#[test]
	fn leaves_a_filter_alone_when_its_list_is_missing() {
		let html = format!(r#"<form data-list-filter="missing">{FIELD}</form>"#);
		assert_eq!(fill_filter_summaries(&html), html);
	}
}

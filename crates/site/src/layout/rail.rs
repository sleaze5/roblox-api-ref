use maud::{Markup, html};

pub struct RailEntry {
	pub id: String,
	pub label: String,
}

pub struct RailLink {
	pub id: String,
	pub label: String,
	pub count: Option<usize>,
	pub count_hint: Option<String>,
	pub entries: Vec<RailEntry>,
}

impl RailLink {
	pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
		Self {
			id: id.into(),
			label: label.into(),
			count: None,
			count_hint: None,
			entries: Vec::new(),
		}
	}

	#[must_use]
	pub const fn count(mut self, count: usize) -> Self {
		self.count = Some(count);
		self
	}

	#[must_use]
	pub fn count_hint(mut self, hint: impl Into<String>) -> Self {
		self.count_hint = Some(hint.into());
		self
	}

	#[must_use]
	pub fn entries(mut self, entries: Vec<RailEntry>) -> Self {
		self.entries = entries;
		self
	}
}

/// The "On this page" navigation.
pub fn page_rail(links: &[RailLink]) -> Markup {
	html! {
		nav data-rail="" aria-labelledby="rail-title" class="flex flex-col gap-1" {
			p id="rail-title" class="px-2 pb-1 text-xs font-medium text-fg-muted" { "On this page" }
			@for link in links {
				div data-rail-section=(link.id) class="group/rail flex flex-col" {
					a href={ "#" (link.id) } class="flex h-7 items-center justify-between rounded-sm px-2 text-fg-muted motion-colors group-data-active/rail:text-fg hover:bg-hover hover:text-fg" {
						span { (link.label) }
						@if let Some(count) = link.count {
							span data-tooltip=[link.count_hint.as_deref()] class="text-xs text-fg-faint" { (count) }
						}
					}
					@if !link.entries.is_empty() {
						ul data-rail-entries="" class="ml-2 flex flex-col border-l border-line-subtle pl-1" {
							@for entry in &link.entries {
								li {
									a href={ "#" (entry.id) } data-rail-entry=(entry.id) class="flex h-7 items-center rounded-sm px-2 font-mono text-fg-muted motion-colors hover:bg-hover hover:text-fg data-active:bg-active data-active:text-fg-bright" {
										span class="truncate" { (entry.label) }
									}
								}
							}
						}
					}
				}
			}
		}
	}
}

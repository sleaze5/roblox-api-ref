use maud::{Markup, html};

use crate::components::page::{HeaderIcon, PageHeader, page_lead, root_crumbs};
use crate::context::Context;
use crate::html::{EXTERNAL_REL, EXTERNAL_TARGET};
use crate::layout::Shell;
use crate::routes::SitePageName;
use crate::site_pages::{Block, Inline, is_external, page_content, site_page};

pub fn inline_content(content: &[Inline]) -> Markup {
	html! {
		@for part in content {
			@match part {
				Inline::Text(text) => (text),
				Inline::Code(code) => code { (code) },
				Inline::Link { text, href } => {
					@if is_external(href) {
						a href=(href) target=(EXTERNAL_TARGET) rel=(EXTERNAL_REL) {
							(text)
							span class="sr-only" { " (opens in a new tab)" }
						}
					} @else {
						a href=(href) { (text) }
					}
				}
			}
		}
	}
}

fn content_block(block: &Block) -> Markup {
	html! {
		@match block {
			Block::Paragraph(content) => p { (inline_content(content)) },
			Block::List(items) => ul {
				@for item in items {
					li { (inline_content(item)) }
				}
			},
		}
	}
}

pub fn site_page_body(cx: &Context, name: SitePageName) -> Markup {
	let page = site_page(name);
	let content = page_content(name, &cx.api.version, &cx.api.hash);
	let header = PageHeader {
		crumbs: root_crumbs(),
		icon: HeaderIcon::Named(page.icon),
		title: html! { (page.title) },
		code: false,
		docs: None,
	};
	let body = html! {
		div class="flex flex-col gap-10" {
			(header.render(cx, Some(page_lead(&html! { (content.lead) }))))
			@for section in &content.sections {
				@let title_id = format!("{}-title", section.id);
				section class="flex flex-col gap-3" aria-labelledby=(title_id) {
					h2 id=(title_id) class="text-lg font-semibold" { (section.heading) }
					div class="doc-text flex max-w-prose flex-col gap-3 text-base text-fg" {
						@for block in &section.blocks { (content_block(block)) }
					}
				}
			}
		}
	};
	Shell::new(None).render(cx, &body)
}

//! The Atom feed of the newest updates.

use std::fmt::Write as _;

use rapi_core::api::release::release_name;
use rapi_core::api::text::update_summary;
use rapi_core::model::Update;
use rapi_routes::{FEED, UPDATES, update_href};

use crate::context::Context;
use crate::head::escape_html;
use crate::meta::SITE_NAME;

const FEED_LENGTH: usize = 20;

fn entry(cx: &Context, update: &Update) -> String {
	let url = escape_html(&cx.absolute_url(&update_href(&update.version)));
	let summary = format!(
		"{} {} added, {} removed, and {} changed.",
		update_summary(update),
		update.counts.added,
		update.counts.removed,
		update.counts.changed
	);
	format!(
		"<entry><title>{}</title><id>{url}</id><link href=\"{url}\"/><updated>{}</updated><summary>{}</summary></entry>",
		escape_html(&release_name(&update.version)),
		update.published,
		escape_html(&summary)
	)
}

pub fn updates_feed(cx: &Context) -> String {
	let recent = &cx.updates[..FEED_LENGTH.min(cx.updates.len())];
	let updated = recent.first().map_or("1970-01-01T00:00:00.000Z", |update| {
		update.published.as_str()
	});
	let feed = escape_html(&cx.absolute_url(FEED));
	let mut xml = format!(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?><feed xmlns=\"http://www.w3.org/2005/Atom\"><title>{}</title><id>{feed}</id><link rel=\"self\" href=\"{feed}\"/><link href=\"{}\"/><updated>{updated}</updated><author><name>{}</name></author>",
		escape_html(&format!("{SITE_NAME} updates")),
		escape_html(&cx.absolute_url(UPDATES)),
		escape_html(SITE_NAME)
	);
	for update in recent {
		xml.push_str(&entry(cx, update));
	}
	let _ = writeln!(xml, "</feed>");
	xml
}

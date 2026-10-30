use maud::{Markup, html};
use rapi_core::api::text::{REBUILD_TEXT, UPDATES_LEAD, update_summary};
use rapi_core::icons::Icon;
use rapi_core::js::{format_date, format_month, format_short_date};
use rapi_core::model::{ChangeStatus, OwnerChanges, Release, Update};
use rapi_routes::{owner_anchor, update_anchor, update_href, upload_name};

use crate::components::facts::{fact, fact_list};
use crate::components::filters::{ListFilterPlacement, list_filter};
use crate::components::page::{HeaderIcon, PageHeader, page_lead, root_crumbs, updates_crumbs};
use crate::components::updates::{
	OwnerSubsection, owner_icon, owner_sections, owner_subsection_block,
};
use crate::context::Context;
use crate::html::{IconSize, class_icon_style, icon};
use crate::layout::{RailEntry, RailLink, Section, Shell, page_rail, section_sidebar};
use crate::pages::index_pages::{INDEX_GRID_CLASS, index_row_markup, index_section_markup};

const RECENT_COUNT: usize = 10;

struct UpdateMonth<'a> {
	title: String,
	updates: Vec<&'a Update>,
}

struct UpdateYear<'a> {
	year: String,
	count: usize,
	months: Vec<UpdateMonth<'a>>,
}

fn update_calendar(updates: &[Update]) -> Vec<UpdateYear<'_>> {
	let mut years: Vec<UpdateYear> = Vec::new();
	for update in updates {
		let year = update.published.get(..4).unwrap_or_default();
		let month = format_month(&update.published);
		if years.last().is_none_or(|last| last.year != year) {
			years.push(UpdateYear {
				year: year.to_owned(),
				count: 0,
				months: Vec::new(),
			});
		}
		let year_group = years.last_mut().expect("a year was just pushed");
		if year_group
			.months
			.last()
			.is_none_or(|last| last.title != month)
		{
			year_group.months.push(UpdateMonth {
				title: month,
				updates: Vec::new(),
			});
		}
		year_group
			.months
			.last_mut()
			.expect("a month was just pushed")
			.updates
			.push(update);
		year_group.count += 1;
	}
	years
}

pub fn update_sidebar(cx: &Context, current: Option<&str>) -> Markup {
	let years = update_calendar(&cx.updates);
	let open_month: Option<(usize, usize)> = match current {
		None => (!years.is_empty() && !years[0].months.is_empty()).then_some((0, 0)),
		Some(version) => years.iter().enumerate().find_map(|(year_index, year)| {
			year.months
				.iter()
				.position(|month| month.updates.iter().any(|update| update.version == version))
				.map(|month_index| (year_index, month_index))
		}),
	};
	let tree = html! {
		ul id="sidebar-tree" data-tree="" data-sticky-tree="" class="tree-sticky class-tree tree-update-icons" {
			@for (year_index, year) in years.iter().enumerate() {
				li {
					details open[open_month.is_some_and(|(open_year, _)| open_year == year_index)] {
						summary style="--depth: 0" {
							span { (year.year) }
							small { (year.count) }
						}
						ul {
							@for (month_index, month) in year.months.iter().enumerate() {
								li {
									details open[open_month == Some((year_index, month_index))] {
										summary style="--depth: 1" {
											span { (month.title) }
											small { (month.updates.len()) }
										}
										ul {
											@for update in &month.updates {
												@let upload = upload_name(&update.hash);
												li data-name=(update.version) data-aliases=(upload) {
													a href=(update_href(&update.version)) aria-current=[(current == Some(update.version.as_str())).then_some("page")] data-tooltip=(upload) {
														span { (update.version) }
														small { (format_short_date(&update.published)) }
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
			}
		}
	};
	section_sidebar(cx, "Filter updates", "sidebar-tree", &tree)
}

fn version_text(release: &Release) -> Markup {
	html! { span class="font-mono" { (release.version) " (" (upload_name(&release.hash)) ")" } }
}

const fn count_hint(status: ChangeStatus) -> &'static str {
	match status {
		ChangeStatus::Added => "Classes, members, enums, and enum items that this version adds.",
		ChangeStatus::Removed => {
			"Classes, members, enums, and enum items that this version removes."
		}
		ChangeStatus::Changed => {
			"Classes, members, enums, and enum items whose signature, tags, or other details changed."
		}
	}
}

fn update_facts(cx: &Context, update: &Update, published: bool) -> Markup {
	let known = cx
		.updates
		.iter()
		.any(|other| other.version == update.previous.version);
	fact_list(&html! {
		@if published {
			(fact("Published", Some("When Roblox published the API files of this version."), &html! { (format_date(&update.published)) }))
		}
		(fact("Hash", Some("The Studio upload that this version comes from."), &html! { span class="font-mono" { (upload_name(&update.hash)) } }))
		(fact("Compared with", Some("The previous version that this update compares against."), &html! {
			@if known {
				a href=(update_href(&update.previous.version)) class="hover:underline" { (version_text(&update.previous)) }
			} @else {
				(version_text(&update.previous))
			}
		}))
		@for status in ChangeStatus::ALL {
			(fact(status.name(), Some(count_hint(status)), &html! { (update.counts.get(status)) }))
		}
		@match update.description_count {
			Some(count) => { (fact("Descriptions", Some("Descriptions that this version adds, changes, or removes."), &html! { (count) })) }
			None => { (fact("Descriptions", Some("The API docs of this version and the one before it are not archived, so description changes are unknown."), &html! { "Not recorded" })) }
		}
	})
}

fn entry_counts(owner: &OwnerChanges) -> rapi_core::model::ChangeCounts {
	let mut counts = rapi_core::model::ChangeCounts::default();
	for entry in owner.entries.iter().filter(|entry| entry.is_api_change()) {
		counts.bump(entry.status);
	}
	counts
}

pub fn change_counts(counts: &rapi_core::model::ChangeCounts, changes: bool) -> Markup {
	let parts = [
		(ChangeStatus::Added, "+", "Added", "text-success"),
		(ChangeStatus::Removed, "−", "Removed", "text-danger"),
		(ChangeStatus::Changed, "~", "Changed", "text-fg-faint"),
	];
	let shown: Vec<_> = parts
		.into_iter()
		.filter(|(status, ..)| {
			counts.get(*status) > 0 && (changes || *status != ChangeStatus::Changed)
		})
		.collect();
	html! {
		@if !shown.is_empty() {
			span class="flex gap-2 tabular-nums" {
				@for (status, mark, label, class) in shown {
					span {
						span aria-hidden="true" class={ "font-mono " (class) } { (mark) }
						span class="sr-only" { (label) " " }
						(counts.get(status))
					}
				}
			}
		}
	}
}

fn owner_note(owner: &OwnerChanges) -> Markup {
	let counts = entry_counts(owner);
	if counts.total() > 0 {
		return change_counts(&counts, true);
	}
	if !owner.details.is_empty() {
		return html! { span { "Details changed" } };
	}
	if owner.status.is_none() {
		html! { span { "Description changed" } }
	} else {
		html! {}
	}
}

fn group_summary(cx: &Context, update: &Update, group: &OwnerSubsection) -> Markup {
	let id = format!("{}-{}", update.version, group.id);
	html! {
		section id=(id) data-list-section="" aria-labelledby={ (id) "-title" } class="flex flex-col gap-2" {
			h3 id={ (id) "-title" } class="flex items-center gap-2 border-b border-line-subtle pb-2 text-base font-semibold" {
				(group.rail_label)
				span class="text-sm font-normal text-fg-faint" { (group.owners.len()) }
			}
			ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2" {
				@for owner in &group.owners {
					li data-name=(owner.name) {
						a href={ (update_href(&update.version)) "#" (owner_anchor(owner.kind, &owner.name)) } class="flex h-7 items-center justify-between gap-3 rounded-sm px-2 motion-colors hover:bg-hover" {
							span class="flex min-w-0 items-center gap-2" {
								span aria-hidden="true" data-kind=(owner.kind.name()) style=[class_icon_style(owner_icon(cx, owner))] class="kind-icon text-fg-muted" {}
								code class="truncate text-code-type" { (owner.name) }
							}
							span class="shrink-0 text-xs text-fg-faint" { (owner_note(owner)) }
						}
					}
				}
			}
		}
	}
}

fn update_summary_section(cx: &Context, update: &Update) -> Markup {
	let id = update_anchor(&update.version);
	html! {
		section id=(id) data-list-section="" aria-labelledby={ (id) "-title" } class="flex flex-col gap-4" {
			div class="flex flex-col gap-2" {
				h2 id={ (id) "-title" } class="flex flex-wrap items-center gap-x-2 text-lg font-semibold" {
					(icon(cx, Icon::GitDiff, IconSize::S20, "text-fg-muted"))
					a href=(update_href(&update.version)) class="font-mono hover:text-fg-bright hover:underline" { (update.version) }
					span class="text-sm font-normal text-fg-faint" { (format_date(&update.published)) }
				}
				(update_facts(cx, update, false))
			}
			@if update.owners.is_empty() {
				p class="text-fg-muted" { (REBUILD_TEXT) }
			} @else {
				div class="flex flex-col gap-6" {
					@for section in owner_sections(update) {
						@for group in &section.groups { (group_summary(cx, update, group)) }
					}
				}
			}
		}
	}
}

pub fn updates_page(cx: &Context) -> Markup {
	let updates = &cx.updates;
	let split = RECENT_COUNT.min(updates.len());
	let (recent, earlier) = updates.split_at(split);
	let mut links: Vec<RailLink> = recent
		.iter()
		.map(|update| {
			RailLink::new(update_anchor(&update.version), &update.version)
				.count(update.counts.total() as usize)
				.count_hint("Changes in this update.")
		})
		.collect();
	if !earlier.is_empty() {
		links.push(RailLink::new("earlier-updates", "Earlier updates").count(earlier.len()));
	}
	let header = PageHeader {
		crumbs: root_crumbs(),
		icon: HeaderIcon::Named(Icon::GitDiff),
		title: html! { "Updates" },
		code: false,
		docs: None,
	};
	let body = html! {
		article class="flex flex-col gap-6" {
			(header.render(cx, Some(page_lead(&html! { (UPDATES_LEAD) }))))
			@if updates.is_empty() {
				p class="text-fg-muted" { "There are no updates yet. An update appears here when the site moves to a new Roblox Studio version." }
			} @else {
				(list_filter(cx, &format!("Filter the {RECENT_COUNT} newest updates"), "update-changes", "q", ListFilterPlacement::Page, Some("classes and enums")))
				div id="update-changes" class="flex flex-col gap-12" {
					@for update in recent { (update_summary_section(cx, update)) }
				}
				@if !earlier.is_empty() {
					(index_section_markup(cx, "earlier-updates", "Earlier updates", Icon::GitDiff, earlier.len(), None, &html! {
						ul class=(INDEX_GRID_CLASS) {
							@for update in earlier {
								(index_row_markup(&update.version, &update_href(&update.version), update.counts.total() as usize, false))
							}
						}
					}))
				}
			}
		}
	};
	Shell::new(Some(Section::Updates))
		.sidebar(update_sidebar(cx, None))
		.rail(page_rail(&links))
		.render(cx, &body)
}

pub fn update_page(cx: &Context, update: &Update) -> Markup {
	let sections = owner_sections(update);
	let links: Vec<RailLink> = sections
		.iter()
		.flat_map(|section| {
			section.groups.iter().map(|group| {
				RailLink::new(&group.id, &group.rail_label)
					.count(group.owners.len())
					.entries(
						group
							.owners
							.iter()
							.map(|owner| RailEntry {
								id: owner_anchor(owner.kind, &owner.name),
								label: owner.name.clone(),
							})
							.collect(),
					)
			})
		})
		.collect();
	let header = PageHeader {
		crumbs: updates_crumbs(),
		icon: HeaderIcon::Named(Icon::GitDiff),
		title: html! { (update.version) },
		code: true,
		docs: None,
	};
	let body = html! {
		article class="flex flex-col gap-6" {
			(header.render(cx, Some(html! {
				(page_lead(&html! { (update_summary(update)) }))
				(update_facts(cx, update, true))
			})))
			@if sections.is_empty() {
				p class="text-fg-muted" { (REBUILD_TEXT) }
			} @else {
				(list_filter(cx, "Filter changes", "update-changes", "q", ListFilterPlacement::Page, None))
				div id="update-changes" data-changes="" class="flex flex-col gap-8" {
					@for section in &sections {
						(index_section_markup(cx, section.id, section.title, section.icon, section.count, None, &html! {
							div class="flex flex-col gap-8" {
								@for group in &section.groups { (owner_subsection_block(cx, group, None, "")) }
							}
						}))
					}
				}
			}
		}
	};
	Shell::new(Some(Section::Updates))
		.sidebar(update_sidebar(cx, Some(&update.version)))
		.rail(page_rail(&links))
		.render(cx, &body)
}

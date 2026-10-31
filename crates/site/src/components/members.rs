use maud::{Markup, html};
use rapi_core::api::code::{text_length, unlinked};
use rapi_core::api::docs_links::class_docs_href;
use rapi_core::api::filter_tags::{FilterTagGroup, member_tag_keys};
use rapi_core::api::history_text::HistoryNoun;
use rapi_core::api::luau::type_tokens;
use rapi_core::api::members::{category_groups, is_deprecated, member_kind_title, members_of_kind};
use rapi_core::api::signature::{
	SignatureParts, inline_tokens, is_nothing, parameter_tokens, signature_parts,
};
use rapi_core::api::tags::member_tags;
use rapi_core::icons::Icon;
use rapi_core::model::{Class, Doc, Member, MemberDetail};
use rapi_routes::{
	FacetKind, MemberKind, OwnerKind, change_anchor, class_href, member_anchor, member_href,
};

use crate::components::code::code_tokens;
use crate::components::doc::doc_text;
use crate::components::filters::{FieldCount, filter_field};
use crate::components::history::history_list;
use crate::components::sample::code_sample;
use crate::components::tags::{dated_tags, tag_list};
use crate::context::Context;
use crate::html::{IconSize, creator_hub_link, icon, sprite_icon};

pub const fn kind_icon(kind: MemberKind) -> Icon {
	match kind {
		MemberKind::Property => Icon::SlidersHorizontal,
		MemberKind::Method => Icon::Function,
		MemberKind::Event => Icon::Lightning,
		MemberKind::Callback => Icon::ArrowUUpLeft,
	}
}

pub fn section_id(kind: MemberKind) -> String {
	member_kind_title(kind).to_lowercase()
}

const WRAP_LENGTH: usize = 80;

pub fn signature(parts: &SignatureParts) -> Markup {
	let parameters = parts.parameters.as_deref().unwrap_or_default();
	let wrapped = parameters.len() >= 2 && text_length(&inline_tokens(parts)) > WRAP_LENGTH;
	html! {
		code class="member-code" {
			(code_tokens(&parts.owner))
			wbr;
			(code_tokens(&parts.head))
			@if wrapped {
				@for parameter in parameters {
					span class="block pl-4" { (code_tokens(parameter)) "," }
				}
			} @else {
				@if parts.parameters.is_some() { wbr; }
				@for (index, parameter) in parameters.iter().enumerate() {
					@if index > 0 { ", " }
					(code_tokens(parameter))
				}
			}
			(code_tokens(&parts.tail))
		}
	}
}

struct DetailRow<'a> {
	code: Vec<rapi_core::api::code::CodeToken>,
	description: Option<&'a Doc>,
}

fn detail_table(cx: &Context, caption: &str, heading: &str, rows: &[DetailRow]) -> Markup {
	let described = rows.iter().any(|row| row.description.is_some());
	html! {
		@if !rows.is_empty() {
			div class="detail-table" {
				table {
					caption { (caption) }
					@if described {
						colgroup { col data-code=""; col; }
					}
					thead {
						tr {
							th scope="col" { (heading) }
							@if described { th scope="col" { "Description" } }
						}
					}
					tbody {
						@for row in rows {
							tr {
								td data-code="" { code { (code_tokens(&row.code)) } }
								@if described { td { (doc_text(cx, row.description, "")) } }
							}
						}
					}
				}
			}
		}
	}
}

fn member_details(cx: &Context, member: &Member) -> Markup {
	let parameters: Vec<DetailRow> = member
		.parameters()
		.iter()
		.map(|parameter| DetailRow {
			code: parameter_tokens(parameter),
			description: parameter.description.as_ref(),
		})
		.collect();
	let returns: Vec<DetailRow> = match &member.detail {
		MemberDetail::Method {
			returns,
			return_description,
			..
		}
		| MemberDetail::Callback {
			returns,
			return_description,
			..
		} if !(is_nothing(returns) && return_description.is_none()) => {
			vec![DetailRow {
				code: type_tokens(returns),
				description: return_description.as_ref(),
			}]
		}
		_ => Vec::new(),
	};
	html! {
		(detail_table(cx, "Parameters", "Parameter", &parameters))
		(detail_table(cx, "Returns", "Type", &returns))
	}
}

fn preferred(member: &Member) -> Markup {
	html! {
		@if let Some(name) = &member.preferred {
			p class="text-fg-muted" {
				"Use "
				@match &member.preferred_target {
					None => span class="font-mono text-fg" { (name) },
					Some(target) => a href=(member_href(&target.owner, target.kind, name)) class="font-mono text-fg underline decoration-line-strong hover:decoration-fg" { (name) },
				}
				" instead."
			}
		}
	}
}

fn member_card(cx: &Context, member: &Member, owner: &str, own: bool) -> Markup {
	let kind = member.kind();
	let anchor = member_anchor(kind, &member.name);
	let href = if own {
		format!("#{anchor}")
	} else {
		member_href(owner, kind, &member.name)
	};
	let mut parts = signature_parts(member, owner, (!own).then_some(href.as_str()));
	if own {
		parts.owner = unlinked(&parts.owner);
	}
	let tags = member_tag_keys(member).join(" ");
	html! {
		li data-member="" data-kind=(kind.name()) data-name=(member.name)
			data-deprecated=[is_deprecated(&member.tags).then_some("")]
			data-tags=(tags)
			id=[own.then_some(anchor.as_str())]
			class="member-card" {
			div class="member-head" {
				(signature(&parts))
				div class="member-actions" {
					a href=(href) data-copy-link="" data-tooltip="Copy link" aria-label={ "Copy link to " (member.name) } class="copy-link" {}
					(creator_hub_link(&class_docs_href(owner, Some(&member.name)), &member.name))
				}
			}
			(doc_text(cx, member.description.as_ref(), "member-description"))
			(tag_list(&dated_tags(member_tags(member), member.history.as_ref()), ""))
			(member_details(cx, member))
			(preferred(member))
			@if own {
				(history_list(member.history.as_ref(), &change_anchor(OwnerKind::Class, owner, &member.name), HistoryNoun::Member, ""))
				(code_sample(member.sample.as_ref()))
			}
		}
	}
}

fn cards(cx: &Context, members: &[&Member], owner: &str, own: bool) -> Markup {
	html! {
		ul class="flex flex-col gap-4" {
			@for member in members { (member_card(cx, member, owner, own)) }
		}
	}
}

fn group_header(cx: &Context, owner: &str, own: bool, count: usize, id: &str) -> Markup {
	let label = if own { "Defined by" } else { "Inherited from" };
	html! {
		div class="group-head" {
			button type="button" data-collapse-toggle=(owner) aria-expanded=(if own { "true" } else { "false" }) aria-controls=(id) aria-label={ (label) " " (owner) } class="absolute inset-0 rounded-sm" {}
			(sprite_icon(cx, Icon::CaretRight, "icon-14 rotate-90 motion-caret"))
			span { (label) }
			@if own {
				span class="font-mono text-fg" { (owner) }
			} @else {
				a href=(class_href(owner)) class="relative font-mono text-code-type hover:underline" { (owner) }
			}
			span data-count="" class="text-fg-faint" { (count) }
		}
	}
}

fn member_group(
	cx: &Context,
	members: &[&Member],
	owner: &str,
	own: bool,
	section: &str,
) -> Markup {
	let id = format!("{section}-{owner}");
	let categorized = members
		.first()
		.is_some_and(|member| member.kind() == MemberKind::Property);
	html! {
		div data-group="" data-owner=(owner) data-inherited-group=[(!own).then_some("")] data-collapsed=[(!own).then_some("")] class="flex flex-col gap-2" {
			(group_header(cx, owner, own, members.len(), &id))
			div id=(id) data-group-body="" class="flex flex-col gap-4" {
				@if categorized {
					@for (category, list) in category_groups(members.to_vec()) {
						div data-subgroup="" class="flex flex-col gap-2" {
							h3 class="flex items-baseline gap-1.5 px-3 text-xs font-medium text-fg-muted" {
								span class="text-fg-faint" { "Property category" }
								a href=(cx.facet_href(FacetKind::PropertyCategory, &category)) class="hover:text-fg hover:underline" { (category) }
							}
							(cards(cx, &list, owner, own))
						}
					}
				} @else {
					(cards(cx, members, owner, own))
				}
			}
		}
	}
}

pub fn member_section(
	cx: &Context,
	kind: MemberKind,
	item: &Class,
	ancestors: &[&Class],
) -> Markup {
	let mut groups: Vec<(&str, bool, Vec<&Member>)> = ancestors
		.iter()
		.rev()
		.map(|owner| {
			(
				owner.name.as_str(),
				false,
				members_of_kind(&owner.members, kind),
			)
		})
		.collect();
	groups.push((
		item.name.as_str(),
		true,
		members_of_kind(&item.members, kind),
	));
	groups.retain(|(_, _, members)| !members.is_empty());
	if groups.is_empty() {
		return html! {};
	}
	let id = section_id(kind);
	let title = member_kind_title(kind);
	let total: usize = groups.iter().map(|(_, _, members)| members.len()).sum();
	html! {
		section id=(id) data-section=(kind.name()) aria-labelledby={ (id) "-title" } class="flex flex-col gap-3" {
			h2 id={ (id) "-title" } class="flex items-center gap-2 text-lg font-semibold" {
				(icon(cx, kind_icon(kind), IconSize::S20, "text-fg-muted"))
				(title)
				span data-count="" class="text-sm font-normal text-fg-faint" { (total) }
			}
			@for (owner, own, members) in &groups {
				(member_group(cx, members, owner, *own, &id))
			}
			p data-empty="" hidden class="text-fg-muted" { "No " (title.to_lowercase()) " match the filters." }
		}
	}
}

const MENU_ID: &str = "member-filter-menu";

fn filter_menu(cx: &Context, groups: &[FilterTagGroup]) -> Markup {
	html! {
		button type="button" popovertarget=(MENU_ID) class="menu-button menu-anchor" {
			(icon(cx, Icon::FunnelSimple, IconSize::S14, ""))
			"Filter"
			span data-tag-filter-count="" hidden class="count-badge" {}
			(icon(cx, Icon::CaretDown, IconSize::S14, "text-fg-faint"))
		}
		div id=(MENU_ID) popover="auto" role="dialog" aria-label="Filters" class="menu" {
			div class="flex flex-col gap-1 border-b border-line p-1 pb-2" {
				label class="menu-row cursor-pointer" {
					span class="min-w-0 flex-1 truncate" { "Inherited members" }
					span class="switch me-1" {
						input type="checkbox" name="inherited" checked class="sr-only";
					}
				}
				fieldset class="flex items-center justify-between gap-3 px-2" {
					legend class="sr-only" { "Required tags" }
					span aria-hidden="true" class="text-fg-muted" { "Required tags" }
					span class="segmented" {
						label { input type="radio" name="match" value="any" checked class="sr-only"; "Any" }
						label { input type="radio" name="match" value="all" class="sr-only"; "All" }
					}
				}
			}
			div class="relative flex max-h-80 flex-col gap-2 overflow-y-auto overscroll-contain p-1" {
				@for group in groups {
					fieldset {
						legend class="menu-label" { (group.title) }
						ul {
							@for tag in &group.tags {
								li data-tag-row="" class="menu-row" {
									span aria-hidden="true" class="min-w-0 flex-1 truncate" { (tag.short_label) }
									span class="text-xs text-fg-faint" { (tag.count) }
									label data-tooltip="Require" class="tag-toggle" {
										input type="checkbox" name="require" value=(tag.key) disabled[tag.count == 0] class="sr-only";
										span class="sr-only" { "Require " (tag.label) }
										(icon(cx, Icon::CheckCircle, IconSize::S16, ""))
									}
									label data-tooltip="Exclude" class="tag-toggle tag-toggle-exclude" {
										input type="checkbox" name="exclude" value=(tag.key) disabled[tag.count == 0] class="sr-only";
										span class="sr-only" { "Exclude " (tag.label) }
										(icon(cx, Icon::Prohibit, IconSize::S16, ""))
									}
								}
							}
						}
					}
				}
			}
			div class="flex items-center justify-between gap-2 border-t border-line p-2" {
				p class="text-xs text-fg-faint" { "Members need any or all required tags, and none of the excluded ones." }
				button type="button" data-tag-filter-reset="" class="button" { "Reset" }
			}
		}
	}
}

pub fn member_filter_bar(cx: &Context, kinds: &[MemberKind], tags: &[FilterTagGroup]) -> Markup {
	html! {
		form data-member-filter="" role="search" aria-label="Filter members" class="z-10 -mx-3 flex flex-col gap-2 border-b border-line bg-canvas px-3 py-2 md:sticky md:top-header" {
			div class="flex flex-wrap items-center gap-2" {
				(filter_field(cx, "Filter members by name", "Filter members", FieldCount::Total, "min-w-48 flex-1"))
				(filter_menu(cx, tags))
			}
			div class="flex flex-wrap items-center gap-2" {
				fieldset class="flex flex-wrap gap-1" {
					legend class="sr-only" { "Member kinds" }
					@for kind in kinds {
						label class="chip" {
							input type="checkbox" name={ "kind-" (kind.name().to_lowercase()) } checked class="sr-only";
							(member_kind_title(*kind))
						}
					}
				}
				button type="button" data-collapse-all="collapse" class="ml-auto button" { "Collapse all" }
			}
		}
	}
}

use super::release::release_name;
use super::text::format_month_period;
use crate::js::js_round;
use crate::model::{AccessNow, CountKind, CoverageKind};

pub const fn count_label(kind: CountKind) -> &'static str {
	match kind {
		CountKind::Class => "Classes",
		CountKind::Property => "Properties",
		CountKind::Method => "Methods",
		CountKind::Event => "Events",
		CountKind::Callback => "Callbacks",
		CountKind::Enum => "Enums",
		CountKind::EnumItem => "Enum items",
		CountKind::DataType => "Data types",
	}
}

pub fn ref_noun(kind: &str) -> String {
	let noun = match kind {
		"Class" => "Classes",
		"Property" => "Properties",
		"Method" => "Methods",
		"Event" => "Events",
		"Callback" => "Callbacks",
		"Enum" => "Enums",
		"EnumItem" => "Enum items",
		"DataType" => "Data types",
		"Parameter" => "Parameters",
		"Update" => "Updates",
		_ => "Removed items",
	};
	noun.to_lowercase()
}

pub fn stats_lead(version: &str) -> String {
	format!(
		"Numbers about the Roblox Engine API in {}, and how it changed over the updates that this site tracks.",
		release_name(version)
	)
}

pub fn removed_note(history_start: &str) -> String {
	format!(
		"Removed counts what an update removed since Roblox Studio {history_start} and no later update added back. The members and items of a removed class or enum are not counted on their own. A data type counts as removed when a removed or changed member used it and no current member does."
	)
}

pub const TRENDS_NOTE: &str = "Each update compares one Roblox Studio version with the one before it. The members and items of an added class or enum count as added. Changed counts the classes, members, enums, and enum items whose signature, tags, or other details changed.";

pub fn month_label(period: &str) -> String {
	format_month_period(period)
}

pub const MONTHS_NOTE: &str = "Every update, grouped by the month Roblox published it, across all tracked years. Changes counts the classes, members, enums, and enum items that the updates added, removed, or changed.";

pub fn percent_text(part: u32, total: u32) -> String {
	if total == 0 {
		return "0%".to_owned();
	}
	let ratio = f64::from(part) / f64::from(total) * 100.0;
	format!("{}%", crate::js::js_number(js_round(ratio)))
}

pub fn access_lead(now: AccessNow) -> String {
	format!(
		"Game scripts can use {} of the {} current members ({}), and {} ({}) are safe to use in parallel. The first table splits the members by who can use them, and the second shows how each year changed that.",
		now.scriptable,
		now.members,
		percent_text(now.scriptable, now.members),
		now.thread_safe,
		percent_text(now.thread_safe, now.members)
	)
}

pub const TAGS_LEAD: &str = "How many classes, members, enums, and enum items have each tag now, and how often updates gave it or took it away.";

pub const CAPABILITIES_LEAD: &str = "How many members need each capability now, for reading or writing, and how often updates gave it or took it away.";

pub const DOCS_LEAD: &str = "How much of the current API the English API docs describe, and how many descriptions each year added, rewrote, or removed.";

pub const GROWTH_LEAD: &str = "The classes that gained the most members each year, counting members added minus members removed.";

pub const COMEBACKS_LEAD: &str = "Items that an update removed and a later update added back. The second table pairs each update that removed items with the update that restored them.";

pub const fn coverage_label(kind: CoverageKind) -> &'static str {
	match kind {
		CoverageKind::Class => "Classes",
		CoverageKind::Property => "Properties",
		CoverageKind::Method => "Methods",
		CoverageKind::Event => "Events",
		CoverageKind::Callback => "Callbacks",
		CoverageKind::Parameter => "Parameters",
		CoverageKind::Return => "Return values",
		CoverageKind::Enum => "Enums",
		CoverageKind::EnumItem => "Enum items",
	}
}

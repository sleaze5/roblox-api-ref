use super::release::release_name;
use super::text::format_month_period;
use crate::js::{js_number, js_round};
use crate::model::{
	AccessNow, BUCKET_STARTS, ChangeTally, CountKind, CoverageKind, GAP_DAYS, Tallies, TrendKind,
	UpdateTrend,
};

pub const fn count_label(kind: CountKind) -> &'static str {
	match kind {
		CountKind::Class => "Classes",
		CountKind::Service => "Services",
		CountKind::Property => "Properties",
		CountKind::Method => "Methods",
		CountKind::Event => "Events",
		CountKind::Callback => "Callbacks",
		CountKind::Enum => "Enums",
		CountKind::EnumItem => "Enum items",
		CountKind::DataType => "Data types",
	}
}

pub const fn kind_singular(kind: CountKind) -> &'static str {
	match kind {
		CountKind::Class => "class",
		CountKind::Service => "service",
		CountKind::Property => "property",
		CountKind::Method => "method",
		CountKind::Event => "event",
		CountKind::Callback => "callback",
		CountKind::Enum => "enum",
		CountKind::EnumItem => "enum item",
		CountKind::DataType => "data type",
	}
}

pub const fn kind_plural(kind: CountKind) -> &'static str {
	match kind {
		CountKind::Class => "classes",
		CountKind::Service => "services",
		CountKind::Property => "properties",
		CountKind::Method => "methods",
		CountKind::Event => "events",
		CountKind::Callback => "callbacks",
		CountKind::Enum => "enums",
		CountKind::EnumItem => "enum items",
		CountKind::DataType => "data types",
	}
}

pub const fn kind_slug(kind: CountKind) -> &'static str {
	match kind {
		CountKind::Class => "class",
		CountKind::Service => "service",
		CountKind::Property => "property",
		CountKind::Method => "method",
		CountKind::Event => "event",
		CountKind::Callback => "callback",
		CountKind::Enum => "enum",
		CountKind::EnumItem => "enum-item",
		CountKind::DataType => "data-type",
	}
}

pub const SERVICES_HINT: &str = "Classes tagged Service. The Classes row includes them.";

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
		"Name" => "Names",
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
	format!("{}%", js_number(js_round(ratio)))
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

pub fn capitalized(text: &str) -> String {
	let mut chars = text.chars();
	chars.next().map_or_else(String::new, |first| {
		first.to_uppercase().chain(chars).collect()
	})
}

pub const PER_UPDATE_NOTE: &str = "The classes, members, and enum items that each update added and removed. A number marks a bar that runs past the axis.";

pub const PER_UPDATE_TABLE_NOTE: &str = "The classes, members, and enum items that the tracked updates added and removed, in all and on average per update.";

pub const PER_UPDATE_KINDS: [(&str, Option<TrendKind>); 7] = [
	("Members", None),
	("Classes", Some(TrendKind::Class)),
	("Properties", Some(TrendKind::Property)),
	("Methods", Some(TrendKind::Method)),
	("Events", Some(TrendKind::Event)),
	("Callbacks", Some(TrendKind::Callback)),
	("Enum items", Some(TrendKind::EnumItem)),
];

pub fn kind_tally(tallies: &Tallies, kind: Option<TrendKind>) -> ChangeTally {
	kind.map_or_else(|| tallies.member(), |kind| *tallies.get(kind))
}

pub struct PerUpdate {
	pub label: &'static str,
	pub added: u32,
	pub removed: u32,
	pub average_added: String,
	pub average_removed: String,
}

pub fn average_text(value: f64) -> String {
	js_number(js_round(value * 10.0) / 10.0)
}

#[expect(clippy::cast_precision_loss, reason = "update counts are small")]
pub fn per_update_totals(updates: &[UpdateTrend]) -> Vec<PerUpdate> {
	let count = updates.len().max(1) as f64;
	PER_UPDATE_KINDS
		.into_iter()
		.map(|(label, kind)| {
			let added: u32 = updates
				.iter()
				.map(|trend| kind_tally(&trend.tallies, kind).added)
				.sum();
			let removed: u32 = updates
				.iter()
				.map(|trend| kind_tally(&trend.tallies, kind).removed)
				.sum();
			PerUpdate {
				label,
				added,
				removed,
				average_added: average_text(f64::from(added) / count),
				average_removed: average_text(f64::from(removed) / count),
			}
		})
		.collect()
}

pub const DISTRIBUTIONS_LEAD: &str = "How the current API spreads by size: how many classes define each number of properties, methods, events, and callbacks themselves, how many members take each number of parameters, and how many enums have each number of items.";

pub const RELEASE_DAYS_LEAD: &str = "The weekday, in UTC, that Roblox published each tracked update on, and the days from each update to the next.";

pub const WEEKDAYS: [&str; 7] = [
	"Monday",
	"Tuesday",
	"Wednesday",
	"Thursday",
	"Friday",
	"Saturday",
	"Sunday",
];

pub fn bucket_label(index: usize) -> String {
	let start = BUCKET_STARTS[index];
	match BUCKET_STARTS.get(index + 1) {
		None => format!("{start}+"),
		Some(next) if *next == start + 1 => start.to_string(),
		Some(next) => format!("{start}–{}", next - 1),
	}
}

pub fn gap_label(days: u32) -> String {
	if days >= GAP_DAYS {
		format!("{days}+")
	} else {
		days.to_string()
	}
}

pub fn unit_text(value: f64, unit: &str) -> String {
	if value.total_cmp(&1.0).is_eq() {
		unit.strip_suffix('s').unwrap_or(unit).to_owned()
	} else {
		unit.to_owned()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn labels_each_bucket_by_its_range() {
		let labels: Vec<String> = (0..BUCKET_STARTS.len()).map(bucket_label).collect();
		assert_eq!(
			labels,
			["0", "1", "2", "3", "4", "5–9", "10–19", "20–49", "50+"]
		);
		assert_eq!(gap_label(GAP_DAYS), format!("{GAP_DAYS}+"));
	}

	#[test]
	fn writes_a_singular_unit_for_one() {
		assert_eq!(unit_text(1.0, "days"), "day");
		assert_eq!(unit_text(2.0, "days"), "days");
	}
}

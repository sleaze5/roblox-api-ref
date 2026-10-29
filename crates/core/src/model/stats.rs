use rapi_routes::MemberKind;
use serde::Serialize;

use super::RecordGroup;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum CountKind {
	Class,
	Service,
	Property,
	Method,
	Event,
	Callback,
	Enum,
	EnumItem,
	DataType,
}

impl CountKind {
	pub const ALL: [Self; 9] = [
		Self::Class,
		Self::Service,
		Self::Property,
		Self::Method,
		Self::Event,
		Self::Callback,
		Self::Enum,
		Self::EnumItem,
		Self::DataType,
	];

	pub const MEMBERS: [Self; 4] = [Self::Property, Self::Method, Self::Event, Self::Callback];

	pub const ENTRIES: [Self; 5] = [
		Self::Property,
		Self::Method,
		Self::Event,
		Self::Callback,
		Self::EnumItem,
	];

	pub const ITEMS: [Self; 7] = [
		Self::Class,
		Self::Property,
		Self::Method,
		Self::Event,
		Self::Callback,
		Self::Enum,
		Self::EnumItem,
	];

	pub const fn member(kind: MemberKind) -> Self {
		match kind {
			MemberKind::Property => Self::Property,
			MemberKind::Method => Self::Method,
			MemberKind::Event => Self::Event,
			MemberKind::Callback => Self::Callback,
		}
	}

	pub const fn is_subset(self) -> bool {
		matches!(self, Self::Service)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CountRow {
	pub kind: CountKind,
	pub current: u32,
	pub removed: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum TrendKind {
	Class,
	Property,
	Method,
	Event,
	Callback,
	Enum,
	EnumItem,
}

impl TrendKind {
	pub const fn member(kind: MemberKind) -> Self {
		match kind {
			MemberKind::Property => Self::Property,
			MemberKind::Method => Self::Method,
			MemberKind::Event => Self::Event,
			MemberKind::Callback => Self::Callback,
		}
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ChangeTally {
	pub added: u32,
	pub removed: u32,
	pub changed: u32,
	pub deprecated: u32,
}

impl ChangeTally {
	pub const fn add(&mut self, other: &Self) {
		self.added += other.added;
		self.removed += other.removed;
		self.changed += other.changed;
		self.deprecated += other.deprecated;
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tallies {
	pub class: ChangeTally,
	pub property: ChangeTally,
	pub method: ChangeTally,
	pub event: ChangeTally,
	pub callback: ChangeTally,
	#[serde(rename = "Enum")]
	pub enumeration: ChangeTally,
	pub enum_item: ChangeTally,
}

impl Tallies {
	pub const fn get(&self, kind: TrendKind) -> &ChangeTally {
		match kind {
			TrendKind::Class => &self.class,
			TrendKind::Property => &self.property,
			TrendKind::Method => &self.method,
			TrendKind::Event => &self.event,
			TrendKind::Callback => &self.callback,
			TrendKind::Enum => &self.enumeration,
			TrendKind::EnumItem => &self.enum_item,
		}
	}

	pub const fn get_mut(&mut self, kind: TrendKind) -> &mut ChangeTally {
		match kind {
			TrendKind::Class => &mut self.class,
			TrendKind::Property => &mut self.property,
			TrendKind::Method => &mut self.method,
			TrendKind::Event => &mut self.event,
			TrendKind::Callback => &mut self.callback,
			TrendKind::Enum => &mut self.enumeration,
			TrendKind::EnumItem => &mut self.enum_item,
		}
	}

	const fn all(&self) -> [ChangeTally; 7] {
		[
			self.class,
			self.property,
			self.method,
			self.event,
			self.callback,
			self.enumeration,
			self.enum_item,
		]
	}

	pub const fn add(&mut self, other: &Self) {
		self.class.add(&other.class);
		self.property.add(&other.property);
		self.method.add(&other.method);
		self.event.add(&other.event);
		self.callback.add(&other.callback);
		self.enumeration.add(&other.enumeration);
		self.enum_item.add(&other.enum_item);
	}

	pub const fn member(&self) -> ChangeTally {
		let mut total = self.property;
		total.add(&self.method);
		total.add(&self.event);
		total.add(&self.callback);
		total
	}

	pub const fn change_total(&self) -> u32 {
		let all = self.all();
		let mut total = 0;
		let mut index = 0;
		while index < all.len() {
			total += all[index].added + all[index].removed + all[index].changed;
			index += 1;
		}
		total
	}

	pub const fn deprecated_total(&self) -> u32 {
		let all = self.all();
		let mut total = 0;
		let mut index = 0;
		while index < all.len() {
			total += all[index].deprecated;
			index += 1;
		}
		total
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateTrend {
	pub version: String,
	pub hash: String,
	pub published: String,
	pub tallies: Tallies,
	pub classes: u32,
	pub enums: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PeriodTrend {
	pub period: String,
	pub updates: u32,
	pub tallies: Tallies,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct UpdateEvent {
	pub version: String,
	pub published: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ComebackBatch {
	pub removed: UpdateEvent,
	pub returned: UpdateEvent,
	pub count: u32,
	pub days: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPeriod {
	pub period: String,
	pub added_open: u32,
	pub added_restricted: u32,
	pub opened: u32,
	pub restricted: u32,
	pub made_safe: u32,
	pub made_unsafe: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessNow {
	pub members: u32,
	pub scriptable: u32,
	pub thread_safe: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessTier {
	Game,
	Plugin,
	Command,
	Roblox,
}

impl AccessTier {
	pub const ALL: [Self; 4] = [Self::Game, Self::Plugin, Self::Command, Self::Roblox];
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct KindCounts {
	pub property: u32,
	pub method: u32,
	pub event: u32,
	pub callback: u32,
}

impl KindCounts {
	pub const fn bump(&mut self, kind: MemberKind) {
		match kind {
			MemberKind::Property => self.property += 1,
			MemberKind::Method => self.method += 1,
			MemberKind::Event => self.event += 1,
			MemberKind::Callback => self.callback += 1,
		}
	}

	pub const fn total(&self) -> u32 {
		self.property + self.method + self.event + self.callback
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TierRow {
	pub tier: AccessTier,
	pub kinds: KindCounts,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AccessStats {
	pub now: AccessNow,
	pub tiers: Vec<TierRow>,
	pub years: Vec<AccessPeriod>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TagTrend {
	pub name: String,
	pub current: u32,
	pub gained: u32,
	pub lost: u32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first: Option<UpdateEvent>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum CoverageKind {
	Class,
	Property,
	Method,
	Event,
	Callback,
	Parameter,
	Return,
	Enum,
	EnumItem,
}

impl CoverageKind {
	pub const ALL: [Self; 9] = [
		Self::Class,
		Self::Property,
		Self::Method,
		Self::Event,
		Self::Callback,
		Self::Parameter,
		Self::Return,
		Self::Enum,
		Self::EnumItem,
	];

	pub const fn member(kind: MemberKind) -> Self {
		match kind {
			MemberKind::Property => Self::Property,
			MemberKind::Method => Self::Method,
			MemberKind::Event => Self::Event,
			MemberKind::Callback => Self::Callback,
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CoverageRow {
	pub kind: CoverageKind,
	pub described: u32,
	pub total: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DocPeriod {
	pub period: String,
	pub added: u32,
	pub changed: u32,
	pub removed: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DocStats {
	pub coverage: Vec<CoverageRow>,
	pub years: Vec<DocPeriod>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GrowthClass {
	pub name: String,
	pub added: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GrowthPeriod {
	pub period: String,
	pub classes: Vec<GrowthClass>,
}

pub const BUCKET_STARTS: [u32; 9] = [0, 1, 2, 3, 4, 5, 10, 20, 50];

pub const GAP_DAYS: u32 = 14;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Distribution {
	pub id: String,
	pub label: String,
	pub counts: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct WeekdayTrend {
	pub weekday: usize,
	pub updates: u32,
	pub changes: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GapBucket {
	pub days: u32,
	pub updates: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
	pub history_start: String,
	pub counts: Vec<CountRow>,
	pub records: Vec<RecordGroup>,
	pub updates: Vec<UpdateTrend>,
	pub years: Vec<PeriodTrend>,
	pub months: Vec<PeriodTrend>,
	pub comebacks: Vec<ComebackBatch>,
	pub access: AccessStats,
	pub tags: Vec<TagTrend>,
	pub capabilities: Vec<TagTrend>,
	pub docs: DocStats,
	pub growth: Vec<GrowthPeriod>,
	pub distributions: Vec<Distribution>,
	pub weekdays: Vec<WeekdayTrend>,
	pub gaps: Vec<GapBucket>,
}

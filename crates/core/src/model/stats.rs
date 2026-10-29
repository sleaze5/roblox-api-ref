use rapi_routes::MemberKind;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum CountKind {
	Class,
	Property,
	Method,
	Event,
	Callback,
	Enum,
	EnumItem,
	DataType,
}

impl CountKind {
	pub const ALL: [Self; 8] = [
		Self::Class,
		Self::Property,
		Self::Method,
		Self::Event,
		Self::Callback,
		Self::Enum,
		Self::EnumItem,
		Self::DataType,
	];
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CountRow {
	pub kind: CountKind,
	pub current: u32,
	pub removed: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StatRef {
	Class { name: String },
	Enum { name: String },
	DataType { name: String },
	Member { kind: MemberKind, owner: String, name: String },
	EnumItem { owner: String, name: String },
	Parameter { owner: String, member: String, member_kind: MemberKind, name: String },
	Update { version: String, published: String },
	Removed { label: String, version: String, anchor: String },
}

impl StatRef {
	pub const fn kind_name(&self) -> &'static str {
		match self {
			Self::Class { .. } => "Class",
			Self::Enum { .. } => "Enum",
			Self::DataType { .. } => "DataType",
			Self::Member { kind, .. } => kind.name(),
			Self::EnumItem { .. } => "EnumItem",
			Self::Parameter { .. } => "Parameter",
			Self::Update { .. } => "Update",
			Self::Removed { .. } => "Removed",
		}
	}
}

impl Serialize for StatRef {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut map = serializer.serialize_map(None)?;
		map.serialize_entry("kind", self.kind_name())?;
		match self {
			Self::Class { name } | Self::Enum { name } | Self::DataType { name } => {
				map.serialize_entry("name", name)?;
			}
			Self::Member { owner, name, .. } | Self::EnumItem { owner, name } => {
				map.serialize_entry("owner", owner)?;
				map.serialize_entry("name", name)?;
			}
			Self::Parameter {
				owner,
				member,
				member_kind,
				name,
			} => {
				map.serialize_entry("owner", owner)?;
				map.serialize_entry("member", member)?;
				map.serialize_entry("memberKind", member_kind.name())?;
				map.serialize_entry("name", name)?;
			}
			Self::Update { version, published } => {
				map.serialize_entry("version", version)?;
				map.serialize_entry("published", published)?;
			}
			Self::Removed {
				label,
				version,
				anchor,
			} => {
				map.serialize_entry("label", label)?;
				map.serialize_entry("version", version)?;
				map.serialize_entry("anchor", anchor)?;
			}
		}
		map.end()
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatRecord {
	pub id: String,
	pub label: String,
	pub hint: String,
	#[serde(serialize_with = "stat_value")]
	pub value: f64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unit: Option<String>,
	pub items: Vec<StatRef>,
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "serde passes fields by reference")]
fn stat_value<S: Serializer>(value: &f64, serializer: S) -> Result<S::Ok, S::Error> {
	if value.fract() == 0.0 && value.abs() < 1e15 {
		#[expect(clippy::cast_possible_truncation, reason = "the value is a whole number")]
		let whole = *value as i64;
		serializer.serialize_i64(whole)
	} else {
		serializer.serialize_f64(*value)
	}
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RecordGroup {
	pub id: String,
	pub title: String,
	pub records: Vec<StatRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum TrendKind {
	Class,
	Member,
	Enum,
	EnumItem,
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
	pub member: ChangeTally,
	#[serde(rename = "Enum")]
	pub enumeration: ChangeTally,
	pub enum_item: ChangeTally,
}

impl Tallies {
	pub const fn get_mut(&mut self, kind: TrendKind) -> &mut ChangeTally {
		match kind {
			TrendKind::Class => &mut self.class,
			TrendKind::Member => &mut self.member,
			TrendKind::Enum => &mut self.enumeration,
			TrendKind::EnumItem => &mut self.enum_item,
		}
	}

	pub const fn add(&mut self, other: &Self) {
		self.class.add(&other.class);
		self.member.add(&other.member);
		self.enumeration.add(&other.enumeration);
		self.enum_item.add(&other.enum_item);
	}

	pub const fn change_total(&self) -> u32 {
		let mut total = 0;
		let all = [self.class, self.member, self.enumeration, self.enum_item];
		let mut index = 0;
		while index < all.len() {
			total += all[index].added + all[index].removed + all[index].changed;
			index += 1;
		}
		total
	}

	pub const fn deprecated_total(&self) -> u32 {
		self.class.deprecated
			+ self.member.deprecated
			+ self.enumeration.deprecated
			+ self.enum_item.deprecated
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
}

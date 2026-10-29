use std::path::Path;
use std::sync::LazyLock;

use rapi_core::generate::pipeline::{Paths, Site, generate_site};
use rapi_core::model::{
	AccessTier, ApiType, CountKind, CoverageKind, MemberDetail, StatRecord, StatRef,
};
use rapi_routes::FacetKind;

static SITE: LazyLock<Site> = LazyLock::new(|| {
	let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
	generate_site(&Paths::new(root)).expect("the site generates from data/ and diffs/")
});

fn record(id: &str) -> &'static StatRecord {
	SITE.stats
		.records
		.iter()
		.flat_map(|group| &group.records)
		.find(|record| record.id == id)
		.unwrap_or_else(|| panic!("no record {id}"))
}

fn count(kind: CountKind) -> (u32, u32) {
	let row = SITE
		.stats
		.counts
		.iter()
		.find(|row| row.kind == kind)
		.expect("every kind is counted");
	(row.current, row.removed)
}

fn len(count: usize) -> u32 {
	u32::try_from(count).expect("counts fit in 32 bits")
}

fn member_total() -> usize {
	SITE.api
		.classes
		.values()
		.map(|item| item.members.len())
		.sum()
}

#[test]
fn counts_the_current_classes_enums_and_data_types() {
	let api = &SITE.api;
	assert_eq!(
		count(CountKind::Class),
		(len(api.classes.len()), len(api.removed_classes().len()))
	);
	assert_eq!(count(CountKind::Enum).0, len(api.enums.len()));
	assert_eq!(count(CountKind::DataType).0, len(api.data_types.len()));
}

#[test]
fn counts_every_current_member_once_by_kind() {
	let counted: u32 = [
		CountKind::Property,
		CountKind::Method,
		CountKind::Event,
		CountKind::Callback,
	]
	.into_iter()
	.map(|kind| count(kind).0)
	.sum();
	assert_eq!(counted, len(member_total()));
}

#[test]
fn names_instance_as_the_class_with_the_most_subclasses() {
	let found = record("most-subclasses");
	assert_eq!(
		found.items,
		[StatRef::Class {
			name: "Instance".to_owned()
		}]
	);
	#[expect(clippy::cast_precision_loss, reason = "a subclass count")]
	let subclasses = SITE.api.classes["Instance"].subclasses.len() as f64;
	assert!((found.value - subclasses).abs() < f64::EPSILON);
}

#[test]
fn lists_every_class_that_ties_for_the_longest_name() {
	let longest = SITE
		.api
		.classes
		.keys()
		.map(String::len)
		.max()
		.unwrap_or_default();
	let ties = SITE
		.api
		.classes
		.keys()
		.filter(|name| name.len() == longest)
		.count();
	let found = record("longest-class-name");
	#[expect(clippy::cast_precision_loss, reason = "a name length")]
	let expected = longest as f64;
	assert!((found.value - expected).abs() < f64::EPSILON);
	assert_eq!(found.items.len(), ties);
}

#[test]
fn follows_the_class_count_back_through_every_update_oldest_first() {
	let trends = &SITE.stats.updates;
	assert_eq!(trends.len(), SITE.updates.len());
	assert_eq!(
		trends.last().map(|trend| &trend.version),
		SITE.updates.first().map(|update| &update.version)
	);
	assert_eq!(
		trends.last().map(|trend| trend.classes),
		Some(len(SITE.api.classes.len()))
	);
	for pair in trends.windows(2) {
		let tally = pair[1].tallies.class;
		assert_eq!(
			pair[1].classes,
			pair[0].classes + tally.added - tally.removed,
			"{}",
			pair[1].version
		);
	}
}

#[test]
fn sums_the_updates_of_each_year_and_of_each_month() {
	let added: u32 = SITE
		.stats
		.updates
		.iter()
		.map(|trend| trend.tallies.member.added)
		.sum();
	for periods in [&SITE.stats.years, &SITE.stats.months] {
		assert_eq!(
			periods.iter().map(|period| period.updates).sum::<u32>(),
			len(SITE.updates.len())
		);
		assert_eq!(
			periods
				.iter()
				.map(|period| period.tallies.member.added)
				.sum::<u32>(),
			added
		);
	}
}

#[test]
fn groups_months_across_years_from_january_to_december() {
	let months: Vec<&str> = SITE
		.stats
		.months
		.iter()
		.map(|month| month.period.as_str())
		.collect();
	let mut sorted = months.clone();
	sorted.sort_unstable();
	assert_eq!(months, sorted);
	assert!(months.len() <= 12);
}

#[test]
fn counts_every_comeback_once_in_a_batch_of_updates() {
	let batches: u32 = SITE.stats.comebacks.iter().map(|batch| batch.count).sum();
	assert!((f64::from(batches) - record("items-added-back").value).abs() < f64::EPSILON);
}

#[test]
fn splits_each_years_added_members_by_whether_scripts_can_use_them() {
	for year in &SITE.stats.access.years {
		let trend = SITE
			.stats
			.years
			.iter()
			.find(|trend| trend.period == year.period)
			.expect("a matching year");
		assert_eq!(
			year.added_open + year.added_restricted,
			trend.tallies.member.added,
			"{}",
			year.period
		);
	}
}

#[test]
fn counts_each_tag_as_many_times_as_the_index_lists_its_uses() {
	for (name, facet) in SITE.api.facets.of(FacetKind::Tag) {
		let row = SITE
			.stats
			.tags
			.iter()
			.find(|row| &row.name == name)
			.expect("every tag has a row");
		assert_eq!(row.current, len(facet.uses.len()), "{name}");
	}
}

#[test]
fn measures_description_coverage_against_every_current_class_and_enum() {
	let total = |kind: CoverageKind| {
		SITE.stats
			.docs
			.coverage
			.iter()
			.find(|row| row.kind == kind)
			.map(|row| row.total)
	};
	assert_eq!(
		total(CoverageKind::Class),
		Some(len(SITE.api.classes.len()))
	);
	assert_eq!(total(CoverageKind::Enum), Some(len(SITE.api.enums.len())));
}

#[test]
fn lists_only_classes_that_grew_largest_first() {
	for year in &SITE.stats.growth {
		let added: Vec<i64> = year.classes.iter().map(|item| item.added).collect();
		assert!(added.iter().all(|value| *value > 0), "{}", year.period);
		assert!(
			added.windows(2).all(|pair| pair[0] >= pair[1]),
			"{}",
			year.period
		);
	}
}

#[test]
fn puts_every_current_member_in_exactly_one_access_group() {
	let tiers = &SITE.stats.access.tiers;
	let grouped: u32 = tiers.iter().map(|row| row.kinds.total()).sum();
	assert_eq!(grouped, len(member_total()));
	assert_eq!(tiers[0].tier, AccessTier::Game);
	assert_eq!(tiers[0].kinds.total(), SITE.stats.access.now.scriptable);
}

fn type_names(ty: &ApiType) -> Vec<&str> {
	let mut names = vec![ty.name.as_str()];
	names.extend(ty.members.iter().flatten().flat_map(type_names));
	names
}

#[test]
fn lists_the_members_that_use_a_class_as_their_type() {
	let usages = &SITE.api.classes["BasePart"].used_by;
	assert!(!usages.is_empty());
	for usage in usages {
		let member = SITE.api.classes[&usage.class_name]
			.members
			.iter()
			.find(|member| member.name == usage.member_name)
			.expect("a usage names a current member");
		let types: Vec<&ApiType> = match &member.detail {
			MemberDetail::Property { ty, .. } => vec![ty],
			MemberDetail::Event { .. } => member
				.parameters()
				.iter()
				.map(|parameter| &parameter.ty)
				.collect(),
			MemberDetail::Method { returns, .. } | MemberDetail::Callback { returns, .. } => member
				.parameters()
				.iter()
				.map(|parameter| &parameter.ty)
				.chain([returns])
				.collect(),
		};
		assert!(
			types
				.iter()
				.flat_map(|ty| type_names(ty))
				.any(|name| name == "BasePart"),
			"{}.{}",
			usage.class_name,
			usage.member_name
		);
	}
	assert!(
		SITE.api
			.classes
			.values()
			.any(|item| item.used_by.is_empty())
	);
	assert!(SITE.api.classes["Instance"].used_by.len() > 100);
}

#[test]
fn records_the_history_of_classes_members_and_enums() {
	let api = &SITE.api;
	assert_eq!(
		api.history_start.as_deref(),
		SITE.stats.history_start.as_str().into()
	);
	let recorded = api
		.classes
		.values()
		.filter(|item| item.history.is_some())
		.count();
	assert!(recorded > 0);
	for item in api.classes.values() {
		if let Some(history) = &item.history {
			assert!(
				history.added.is_some() != history.added_before.is_some(),
				"{}",
				item.name
			);
		}
	}
}

#[test]
fn builds_a_search_index_that_decodes_to_the_same_data() {
	let bytes = SITE.search.encode();
	assert_eq!(
		rapi_search::IndexData::decode(&bytes).expect("the index decodes"),
		SITE.search
	);
	assert_eq!(SITE.search.names.len(), SITE.search.kinds.len());
}

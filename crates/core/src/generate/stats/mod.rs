mod access;
mod averages;
mod class_records;
mod comebacks;
mod counts;
mod deprecations;
mod distributions;
mod docs;
mod extremes;
mod growth;
mod history_records;
mod member_records;
mod names;
mod refs;
mod release_days;
mod rhythm;
mod signatures;
mod tags;
mod trends;
mod type_records;

use averages::average_records;

use crate::api::stat_ref::stat_ref_label;
use crate::js::compare_names;
use crate::model::{Api, RecordGroup, Stats, Update};

fn sort_items(mut group: RecordGroup) -> RecordGroup {
	for record in &mut group.records {
		record
			.items
			.sort_by(|a, b| compare_names(&stat_ref_label(a), &stat_ref_label(b)));
	}
	group
}

pub fn build_stats(api: &Api, updates: &[Update], history_start: &str) -> Stats {
	let trends = trends::update_trends(api, updates);
	let comebacks = comebacks::comeback_stats(api, updates);
	let mut records = vec![
		class_records::class_records(api),
		member_records::member_records(api),
		type_records::type_records(api),
		average_records(api),
	];
	records.push(names::name_records(api));
	records.push(history_records::history_records(api, updates));
	records.push(rhythm::rhythm_records(updates));
	records.push(deprecations::deprecation_records(api, updates));
	records.push(comebacks.records);
	records.push(signatures::signature_records(api, updates));
	Stats {
		history_start: history_start.to_owned(),
		counts: counts::count_stats(api, updates),
		records: records.into_iter().map(sort_items).collect(),
		years: trends::year_trends(&trends),
		months: trends::month_trends(&trends),
		updates: trends,
		comebacks: comebacks.batches,
		access: access::access_stats(api, updates),
		tags: tags::tag_stats(api, updates),
		capabilities: tags::capability_stats(api, updates),
		docs: docs::doc_stats(api, updates),
		growth: growth::growth_stats(updates),
		distributions: distributions::distributions(api),
		weekdays: release_days::weekday_trends(updates),
		gaps: release_days::gap_buckets(updates),
	}
}

use crate::api::stats_text::{kind_plural, kind_singular, kind_slug};
use crate::model::{Api, BUCKET_STARTS, CountKind, Distribution};

fn bucket_counts(values: impl Iterator<Item = usize>) -> Vec<u32> {
	let mut counts = vec![0; BUCKET_STARTS.len()];
	for value in values {
		let value = u32::try_from(value).unwrap_or(u32::MAX);
		let index = BUCKET_STARTS
			.iter()
			.rposition(|start| value >= *start)
			.unwrap_or_default();
		counts[index] += 1;
	}
	counts
}

pub fn distributions(api: &Api) -> Vec<Distribution> {
	let mut all: Vec<Distribution> = CountKind::MEMBERS
		.into_iter()
		.map(|kind| Distribution {
			id: format!("{}-per-class", kind_slug(kind)),
			label: format!(
				"{} per class",
				crate::api::stats_text::capitalized(kind_plural(kind))
			),
			counts: bucket_counts(api.classes.values().map(|item| {
				item.members
					.iter()
					.filter(|member| CountKind::member(member.kind()) == kind)
					.count()
			})),
		})
		.collect();
	all.extend(
		[CountKind::Method, CountKind::Event, CountKind::Callback]
			.into_iter()
			.map(|kind| Distribution {
				id: format!("parameters-per-{}", kind_slug(kind)),
				label: format!("Parameters per {}", kind_singular(kind)),
				counts: bucket_counts(
					api.classes
						.values()
						.flat_map(|item| &item.members)
						.filter(|member| CountKind::member(member.kind()) == kind)
						.map(|member| member.parameters().len()),
				),
			}),
	);
	all.push(Distribution {
		id: "items-per-enum".to_owned(),
		label: "Items per enum".to_owned(),
		counts: bucket_counts(api.enums.values().map(|item| item.items.len())),
	});
	all
}

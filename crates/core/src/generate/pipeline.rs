use std::collections::{HashMap, HashSet};
use std::path::Path;

use rayon::prelude::*;

use super::class_icons::{ClassIconFiles, apply_class_icons};
use super::docs::ApiNames;
use super::facets::{FacetSlugs, build_facet_slugs};
use super::history::{apply_history, build_history, unrecorded_key};
use super::normalize::normalize_api;
use super::search_index::build_search_index;
use super::stats::build_stats;
use super::updates::{DatedDiff, build_updates, check_diff};
use crate::dump::{ApiDocs, Dump, DumpDiff, RobloxVersion};
use crate::model::{Api, Stats, Update};
use crate::paths::{DataFile, Paths};

pub struct Site {
	pub api: Api,
	pub updates: Vec<Update>,
	pub stats: Stats,
	pub slugs: FacetSlugs,
	pub class_icons: ClassIconFiles,
	pub search: rapi_search::IndexData,
	pub version: RobloxVersion,
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
	let bytes = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
	serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

fn read_diffs(paths: &Paths, versions: &[RobloxVersion]) -> Result<Vec<DatedDiff>, String> {
	versions
		.par_windows(2)
		.map(|pair| {
			let diff: DumpDiff = read_json(&paths.diff(&pair[1].version))?;
			check_diff(&diff, &pair[0], &pair[1])?;
			Ok(DatedDiff {
				diff,
				published: pair[1].published.clone(),
			})
		})
		.collect()
}

pub fn generate_site(paths: &Paths) -> Result<Site, String> {
	let versions = paths.tracked_versions()?;
	let version = versions[versions.len() - 1].clone();
	let ((dump, docs), (icons, diffs)) = rayon::join(
		|| {
			rayon::join(
				|| read_json::<Dump>(&paths.data_file(DataFile::ApiDump, &version)),
				|| read_json::<ApiDocs>(&paths.data_file(DataFile::ApiDocs, &version)),
			)
		},
		|| {
			rayon::join(
				|| {
					read_json::<HashMap<String, String>>(
						&paths.data_file(DataFile::StudioIcons, &version),
					)
				},
				|| read_diffs(paths, &versions),
			)
		},
	);
	let mut api = normalize_api(&dump?, &docs?, &version, &versions[0].version);
	let class_icons = apply_class_icons(&mut api.classes, &icons?)?;
	let diffs = diffs?;
	let unrecorded: HashSet<String> = diffs
		.iter()
		.flat_map(|dated| &dated.diff.unrecorded)
		.map(unrecorded_key)
		.collect();
	let updates = {
		let names = ApiNames {
			classes: &api.classes,
			enums: &api.enums,
			data_types: &api.data_types,
		};
		build_updates(&diffs, &names)
	};
	let table = build_history(&updates);
	apply_history(&mut api, &table, &unrecorded);
	let stats = build_stats(&api, &updates);
	let slugs = build_facet_slugs(&api.facets);
	let search = build_search_index(&api, &updates);
	Ok(Site {
		api,
		updates,
		stats,
		slugs,
		class_icons,
		search,
		version,
	})
}

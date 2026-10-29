use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use super::class_icons::{ClassIconFiles, apply_class_icons};
use super::docs::ApiNames;
use super::facets::{FacetSlugs, build_facet_slugs};
use super::history::{apply_history, build_history};
use super::normalize::normalize_api;
use super::stats::build_stats;
use super::updates::{DatedDiff, build_updates, check_diff};
use crate::dump::{ApiDocs, Dump, DumpDiff, RobloxVersion, read_versions};
use crate::model::{Api, Stats, Update};

pub struct Paths {
	pub root: PathBuf,
}

impl Paths {
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}

	pub fn data(&self) -> PathBuf {
		self.root.join("data")
	}

	pub fn diffs(&self) -> PathBuf {
		self.root.join("diffs")
	}

	pub fn versions(&self) -> PathBuf {
		self.root.join("roblox-versions.json")
	}

	pub fn generated(&self) -> PathBuf {
		self.root.join("src").join("generated")
	}

	pub fn diff(&self, version: &str) -> PathBuf {
		self.diffs().join(format!("{version}.json"))
	}
}

pub struct Site {
	pub api: Api,
	pub updates: Vec<Update>,
	pub stats: Stats,
	pub slugs: FacetSlugs,
	pub class_icons: ClassIconFiles,
	pub version: RobloxVersion,
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
	let bytes = std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
	serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

pub fn latest_version(paths: &Paths) -> Result<(Vec<RobloxVersion>, RobloxVersion), String> {
	let versions = read_versions(&paths.versions()).map_err(|error| error.to_string())?;
	let version = versions.last().cloned().ok_or_else(|| {
		"roblox-versions.json has no versions. Run `bun vp run dump:update` first.".to_owned()
	})?;
	Ok((versions, version))
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
	let (versions, version) = latest_version(paths)?;
	let data = paths.data();
	let ((dump, docs), (icons, diffs)) = rayon::join(
		|| {
			rayon::join(
				|| read_json::<Dump>(&data.join(version.dump_file_name())),
				|| read_json::<ApiDocs>(&data.join(version.api_docs_file_name())),
			)
		},
		|| {
			rayon::join(
				|| read_json::<HashMap<String, String>>(&data.join(version.studio_icons_file_name())),
				|| read_diffs(paths, &versions),
			)
		},
	);
	let mut api = normalize_api(&dump?, &docs?, &version);
	let class_icons = apply_class_icons(&mut api.classes, &icons?)?;
	let diffs = diffs?;
	let updates = {
		let names = ApiNames {
			classes: &api.classes,
			enums: &api.enums,
			data_types: &api.data_types,
		};
		build_updates(&diffs, &names)
	};
	let history_start = versions
		.first()
		.map_or_else(|| version.version.clone(), |first| first.version.clone());
	let table = build_history(&updates);
	apply_history(&mut api, &table, &history_start);
	let stats = build_stats(&api, &updates, &history_start);
	let slugs = build_facet_slugs(&api.facets);
	Ok(Site {
		api,
		updates,
		stats,
		slugs,
		class_icons,
		version,
	})
}

use std::path::{Path, PathBuf};

use futures_util::{StreamExt, TryStreamExt, stream};
use indexmap::IndexMap;
use rapi_core::dump::{
	ChangeType, DiffVersion, DumpChange, DumpDiff, RobloxVersion, compare_versions, read_versions,
	to_tab_json, write_versions,
};
use rapi_core::generate::pipeline::Paths;
use reqwest::Client;
use serde_json::Value;

use crate::http::{client, get_bytes, get_json};
use crate::zip::Result;
use crate::{CDN, CHANNEL, api_dump_url, diff, runtime, upload_hash};

const ARCHIVE: &str = "https://raw.githubusercontent.com/RobloxAPI/build-archive/master/data";
const GROUPS: [&str; 2] = ["legacy", "production"];
const CONCURRENCY: usize = 16;

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArchiveBuild {
	group: &'static str,
	version: RobloxVersion,
}

fn archive_dump_url(group: &str, hash: &str) -> String {
	format!("{ARCHIVE}/{group}/builds/version-{hash}/API-Dump.json")
}

fn cdn_dump_url(hash: &str) -> String {
	format!("{CDN}/version-{hash}-API-Dump.json")
}

fn utc_iso(rfc3339: &str) -> Option<String> {
	if let Some(local) = rfc3339.strip_suffix('Z') {
		return Some(rapi_core::js::iso_string(rapi_core::js::parse_iso(local)?));
	}
	let (local, zone) = rfc3339.split_at(rfc3339.len().checked_sub(6)?);
	let sign = match zone.as_bytes().first()? {
		b'+' => 1,
		b'-' => -1,
		_ => return None,
	};
	let (hours, minutes) = zone[1..].split_once(':')?;
	let offset = sign * (hours.parse::<i64>().ok()? * 60 + minutes.parse::<i64>().ok()?) * 60_000;
	Some(rapi_core::js::iso_string(
		rapi_core::js::parse_iso(local)? - offset,
	))
}

fn parse_builds(group: &'static str, metadata: &Value) -> Vec<ArchiveBuild> {
	metadata["Builds"]
		.as_array()
		.into_iter()
		.flatten()
		.filter_map(|build| {
			let missing = metadata["Missing"][build["GUID"].as_str()?]
				.as_array()
				.is_some_and(|files| files.iter().any(|file| file == "API-Dump.json"));
			if missing {
				return None;
			}
			Some(ArchiveBuild {
				group,
				version: RobloxVersion {
					channel: CHANNEL.to_owned(),
					version: build["Version"].as_str()?.to_owned(),
					hash: upload_hash(build["GUID"].as_str()?)?.to_owned(),
					published: utc_iso(build["Date"].as_str()?)?,
				},
			})
		})
		.collect()
}

fn select_builds(builds: Vec<ArchiveBuild>, first: &RobloxVersion) -> Vec<ArchiveBuild> {
	let mut unique: IndexMap<String, ArchiveBuild> = IndexMap::new();
	for build in builds {
		if compare_versions(&build.version.version, &first.version).is_ge() {
			continue;
		}
		let earlier = unique
			.get(&build.version.version)
			.is_some_and(|known| known.version.published <= build.version.published);
		if !earlier {
			unique.insert(build.version.version.clone(), build);
		}
	}
	let mut builds: Vec<ArchiveBuild> = unique.into_values().collect();
	builds.sort_by(|a, b| compare_versions(&a.version.version, &b.version.version));
	builds
}

async fn cached_dump(client: &Client, cache: &Path, url: &str, hash: &str) -> Result<Value> {
	let path = cache.join(format!("API-Dump-{hash}.json"));
	let bytes = if let Ok(bytes) = tokio::fs::read(&path).await {
		bytes
	} else {
		let bytes = get_bytes(client, url).await?;
		tokio::fs::write(&path, &bytes)
			.await
			.map_err(|error| format!("{}: {error}", path.display()))?;
		bytes
	};
	serde_json::from_slice(&bytes).map_err(|error| format!("{url}: {error}"))
}

fn full_only(standard: &Value, full: &Value) -> Vec<DumpChange> {
	diff::diff_dump(standard, full)
		.into_iter()
		.filter(|change| change.change == ChangeType::Add)
		.collect()
}

fn diff_between(
	from: &RobloxVersion,
	to: &RobloxVersion,
	before: &Value,
	after: &Value,
	unrecorded: Vec<DumpChange>,
) -> DumpDiff {
	DumpDiff {
		from: DiffVersion {
			version: from.version.clone(),
			hash: from.hash.clone(),
		},
		to: DiffVersion {
			version: to.version.clone(),
			hash: to.hash.clone(),
		},
		changes: diff::diff_dump(before, after),
		descriptions: None,
		unrecorded,
	}
}

fn save(paths: &Paths, diff: &DumpDiff) -> Result<()> {
	let path = paths.diff(&diff.to.version);
	std::fs::write(&path, to_tab_json(diff)).map_err(|error| format!("{}: {error}", path.display()))
}

pub fn backfill(paths: &Paths) -> Result<()> {
	let versions = read_versions(&paths.versions()).map_err(|error| error.to_string())?;
	let first = versions
		.first()
		.cloned()
		.ok_or("roblox-versions.json has no versions. Run `bun vp run dump:update` first.")?;
	let cache: PathBuf = paths.data().join("archive");
	std::fs::create_dir_all(&cache).map_err(|error| error.to_string())?;
	std::fs::create_dir_all(paths.diffs()).map_err(|error| error.to_string())?;
	runtime()?.block_on(async {
		let client = client()?;
		let mut builds = Vec::new();
		for group in GROUPS {
			let metadata = get_json(&client, &format!("{ARCHIVE}/{group}/metadata.json")).await?;
			builds.extend(parse_builds(group, &metadata));
		}
		let builds = select_builds(builds, &first);
		if builds.is_empty() {
			println!("The archive has no builds before {}.", first.version);
			return Ok(());
		}
		println!(
			"Backfilling {} builds from {} to {}.",
			builds.len(),
			builds[0].version.version,
			first.version
		);
		let targets: Vec<(RobloxVersion, String)> = builds
			.iter()
			.map(|build| {
				(
					build.version.clone(),
					archive_dump_url(build.group, &build.version.hash),
				)
			})
			.chain([(first.clone(), cdn_dump_url(&first.hash))])
			.collect();
		let mut dumps = stream::iter(targets.iter().map(|(version, url)| {
			let (client, cache) = (&client, &cache);
			async move {
				Ok::<_, String>((
					version,
					cached_dump(client, cache, url, &version.hash).await?,
				))
			}
		}))
		.buffered(CONCURRENCY);
		let full = get_json(&client, &api_dump_url(&first.hash)).await?;
		let mut previous: Option<(&RobloxVersion, Value)> = None;
		let mut total = 0;
		while let Some((version, dump)) = dumps.try_next().await? {
			if let Some((from, before)) = &previous {
				let unrecorded = if *version == first {
					full_only(&dump, &full)
				} else {
					Vec::new()
				};
				let diff = diff_between(from, version, before, &dump, unrecorded);
				total += diff.changes.len();
				save(paths, &diff)?;
				println!(
					"Saved {} API changes and {} unrecorded APIs from {} to {}.",
					diff.changes.len(),
					diff.unrecorded.len(),
					from.version,
					version.version
				);
			}
			previous = Some((version, dump));
		}
		let count = builds.len();
		let mut all: Vec<RobloxVersion> = builds.into_iter().map(|build| build.version).collect();
		all.extend(versions);
		write_versions(&paths.versions(), &all).map_err(|error| error.to_string())?;
		println!(
			"Backfilled {count} versions with {total} API changes. {} starts the history.",
			all[0].version
		);
		Ok(())
	})
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn converts_archive_dates_to_utc() {
		assert_eq!(
			utc_iso("2011-10-25T23:33:20-07:00").as_deref(),
			Some("2011-10-26T06:33:20.000Z")
		);
		assert_eq!(
			utc_iso("2018-08-07T19:01:13+02:00").as_deref(),
			Some("2018-08-07T17:01:13.000Z")
		);
	}

	#[test]
	fn keeps_the_earliest_build_of_each_version_before_the_first_tracked_one() {
		let metadata = json!({
			"Builds": [
				{ "GUID": "version-aaaaaaaaaaaaaaaa", "Date": "2018-08-14T17:48:03-07:00", "Version": "0.350.0.1" },
				{ "GUID": "version-bbbbbbbbbbbbbbbb", "Date": "2018-08-21T17:26:20-07:00", "Version": "0.351.0.1" },
				{ "GUID": "version-cccccccccccccccc", "Date": "2018-08-22T01:08:01-07:00", "Version": "0.350.0.1" },
				{ "GUID": "version-dddddddddddddddd", "Date": "2018-08-23T01:08:01-07:00", "Version": "0.352.0.1" },
				{ "GUID": "version-eeeeeeeeeeeeeeee", "Date": "2018-08-24T01:08:01-07:00", "Version": "0.353.0.1" },
			],
			"Missing": { "version-dddddddddddddddd": ["API-Dump.json"] },
		});
		let first = RobloxVersion {
			channel: CHANNEL.to_owned(),
			version: "0.353.0.1".to_owned(),
			hash: "ffffffffffffffff".to_owned(),
			published: String::new(),
		};
		let builds = select_builds(parse_builds("production", &metadata), &first);
		let hashes: Vec<&str> = builds
			.iter()
			.map(|build| build.version.hash.as_str())
			.collect();
		assert_eq!(hashes, ["aaaaaaaaaaaaaaaa", "bbbbbbbbbbbbbbbb"]);
	}
}

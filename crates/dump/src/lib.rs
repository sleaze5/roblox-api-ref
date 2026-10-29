pub mod diff;
mod http;
pub mod zip;

use std::path::{Path, PathBuf};

use base64::Engine as _;
use rapi_core::dump::{
	ApiDocs, DiffVersion, DumpDiff, RobloxVersion, compare_versions, read_versions, to_tab_json,
	write_versions,
};
use rapi_core::generate::pipeline::Paths;
use reqwest::Client;
use serde_json::Value;

use crate::http::{HttpReader, client, get_bytes, get_json, last_modified};
use crate::zip::{Result, extract_entries, extract_entry};

const CDN: &str = "https://setup.rbxcdn.com";
const CHANNEL: &str = "LIVE";
const BINARY_TYPE: &str = "WindowsStudio64";
const VERSION_HISTORY_URL: &str = "https://raw.githubusercontent.com/MaximumADHD/Roblox-Client-Tracker/roblox/version-history.json";

pub fn api_dump_url(hash: &str) -> String {
	format!("{CDN}/version-{hash}-Full-API-Dump.json")
}

pub fn api_docs_url(hash: &str) -> String {
	format!("{CDN}/version-{hash}-content-api-docs.zip")
}

pub fn studio_icons_url(hash: &str) -> String {
	format!("{CDN}/version-{hash}-content-studio_svg_textures.zip")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioBuild {
	pub version: String,
	pub hash: String,
}

pub fn upload_hash(upload: &str) -> Option<&str> {
	upload.strip_prefix("version-").filter(|hash| {
		hash.len() == 16
			&& hash
				.bytes()
				.all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
	})
}

pub fn parse_studio_builds(history: &Value) -> Vec<StudioBuild> {
	let mut builds: Vec<StudioBuild> = history
		.as_object()
		.into_iter()
		.flatten()
		.filter_map(|(version, upload)| {
			let hash = upload_hash(upload.as_str()?)?;
			Some(StudioBuild {
				version: version.clone(),
				hash: hash.to_owned(),
			})
		})
		.collect();
	builds.sort_by(|a, b| compare_versions(&a.version, &b.version));
	builds
}

pub fn build_before<'a>(builds: &'a [StudioBuild], build: &StudioBuild) -> Option<&'a StudioBuild> {
	builds
		.iter()
		.rev()
		.find(|entry| compare_versions(&entry.version, &build.version).is_lt())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DataFile {
	ApiDump,
	ApiDocs,
	StudioIcons,
}

impl DataFile {
	const ALL: [Self; 3] = [Self::ApiDump, Self::ApiDocs, Self::StudioIcons];

	const fn prefix(self) -> &'static str {
		match self {
			Self::ApiDump => "Full-API-Dump-",
			Self::ApiDocs => "Api-Docs-",
			Self::StudioIcons => "Studio-Icons-",
		}
	}

	fn file_name(self, version: &RobloxVersion) -> String {
		match self {
			Self::ApiDump => version.dump_file_name(),
			Self::ApiDocs => version.api_docs_file_name(),
			Self::StudioIcons => version.studio_icons_file_name(),
		}
	}

	async fn download(self, client: &Client, hash: &str) -> Result<Vec<u8>> {
		match self {
			Self::ApiDump => get_bytes(client, &api_dump_url(hash)).await,
			Self::ApiDocs => {
				let reader = HttpReader::open(client, &api_docs_url(hash)).await?;
				extract_entry(&reader, "en-us.json").await
			}
			Self::StudioIcons => download_studio_icons(client, hash).await,
		}
	}
}

fn icon_class(name: &str) -> Option<&str> {
	let class = name
		.strip_prefix("Shared\\InsertableObjects\\Dark\\Standard\\")?
		.strip_suffix("@3x.png")?;
	(!class.is_empty() && !class.contains(['\\', '@'])).then_some(class)
}

async fn download_studio_icons(client: &Client, hash: &str) -> Result<Vec<u8>> {
	let url = studio_icons_url(hash);
	let reader = HttpReader::open(client, &url).await?;
	let entries = extract_entries(&reader, |name| icon_class(name).is_some()).await?;
	let icons: serde_json::Map<String, Value> = entries
		.iter()
		.filter_map(|(name, bytes)| {
			let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
			Some((icon_class(name)?.to_owned(), Value::String(encoded)))
		})
		.collect();
	if icons.is_empty() {
		return Err(format!("{url} has no Studio class icons."));
	}
	Ok(Value::Object(icons).to_string().into_bytes())
}

async fn save_file(path: &Path, bytes: &[u8]) -> Result<()> {
	let partial = path.with_extension("partial");
	tokio::fs::write(&partial, bytes)
		.await
		.map_err(|error| format!("{}: {error}", partial.display()))?;
	tokio::fs::rename(&partial, path)
		.await
		.map_err(|error| format!("{}: {error}", path.display()))
}

async fn ensure_file(
	client: &Client,
	data_dir: &Path,
	file: DataFile,
	version: &RobloxVersion,
) -> Result<PathBuf> {
	let file_name = file.file_name(version);
	let path = data_dir.join(&file_name);
	if tokio::fs::try_exists(&path).await.unwrap_or(false) {
		println!("{file_name} is already downloaded.");
		return Ok(path);
	}
	tokio::fs::create_dir_all(data_dir)
		.await
		.map_err(|error| error.to_string())?;
	save_file(&path, &file.download(client, &version.hash).await?).await?;
	println!("Downloaded {file_name}.");
	Ok(path)
}

fn current_version(paths: &Paths) -> Result<RobloxVersion> {
	read_versions(&paths.versions())
		.map_err(|error| error.to_string())?
		.pop()
		.ok_or_else(|| {
			"roblox-versions.json has no versions. Run `bun vp run dump:update` first.".to_owned()
		})
}

fn runtime() -> Result<tokio::runtime::Runtime> {
	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.map_err(|error| error.to_string())
}

pub fn download(paths: &Paths) -> Result<()> {
	let version = current_version(paths)?;
	let data_dir = paths.data();
	runtime()?.block_on(async {
		let client = client()?;
		let downloads = DataFile::ALL.map(|file| ensure_file(&client, &data_dir, file, &version));
		for result in futures_util::future::join_all(downloads).await {
			result?;
		}
		let current: Vec<String> = DataFile::ALL
			.iter()
			.map(|file| file.file_name(&version))
			.collect();
		let mut entries = tokio::fs::read_dir(&data_dir)
			.await
			.map_err(|error| error.to_string())?;
		while let Some(entry) = entries
			.next_entry()
			.await
			.map_err(|error| error.to_string())?
		{
			let name = entry.file_name().to_string_lossy().into_owned();
			let stale = DataFile::ALL
				.iter()
				.any(|file| name.starts_with(file.prefix()))
				&& !current.contains(&name);
			if stale {
				tokio::fs::remove_file(entry.path())
					.await
					.map_err(|error| error.to_string())?;
				println!("Deleted {name}.");
			}
		}
		Ok(())
	})
}

async fn fetch_live_version(client: &Client) -> Result<StudioBuild> {
	let url = format!(
		"https://clientsettings.roblox.com/v2/client-version/{BINARY_TYPE}/channel/{CHANNEL}"
	);
	let response = get_json(client, &url).await.map_err(|error| {
		format!("Failed to fetch the latest {CHANNEL} {BINARY_TYPE} version: {error}")
	})?;
	let version = response["version"]
		.as_str()
		.ok_or("The client version response has no version.")?;
	let upload = response["clientVersionUpload"].as_str().unwrap_or_default();
	let hash =
		upload_hash(upload).ok_or_else(|| format!("Unexpected version upload id: {upload}"))?;
	Ok(StudioBuild {
		version: version.to_owned(),
		hash: hash.to_owned(),
	})
}

async fn published_date(client: &Client, hash: &str) -> Option<String> {
	let (dump_url, docs_url) = (api_dump_url(hash), api_docs_url(hash));
	let (dump, docs) = tokio::join!(
		last_modified(client, &dump_url),
		last_modified(client, &docs_url)
	);
	Some(rapi_core::js::iso_string(dump?.max(docs?)))
}

async fn release(client: &Client, build: &StudioBuild) -> Option<RobloxVersion> {
	Some(RobloxVersion {
		channel: CHANNEL.to_owned(),
		version: build.version.clone(),
		hash: build.hash.clone(),
		published: published_date(client, &build.hash).await?,
	})
}

async fn read_documented(
	client: &Client,
	data_dir: &Path,
	version: &RobloxVersion,
) -> Result<(Value, ApiDocs)> {
	let (dump, docs) = tokio::join!(
		ensure_file(client, data_dir, DataFile::ApiDump, version),
		ensure_file(client, data_dir, DataFile::ApiDocs, version)
	);
	let read = |path: PathBuf| {
		std::fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))
	};
	let dump: Value = serde_json::from_slice(&read(dump?)?).map_err(|error| error.to_string())?;
	let docs: ApiDocs = serde_json::from_slice(&read(docs?)?).map_err(|error| error.to_string())?;
	Ok((dump, docs))
}

pub fn build_diff(
	from: &RobloxVersion,
	to: &RobloxVersion,
	before: (&Value, &ApiDocs),
	after: (&Value, &ApiDocs),
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
		changes: diff::diff_dump(before.0, after.0),
		descriptions: diff::diff_descriptions(before, after),
	}
}

async fn save_diff(
	client: &Client,
	paths: &Paths,
	from: &RobloxVersion,
	to: &RobloxVersion,
) -> Result<()> {
	let data_dir = paths.data();
	let (before, after) = tokio::join!(
		read_documented(client, &data_dir, from),
		read_documented(client, &data_dir, to)
	);
	let ((before_dump, before_docs), (after_dump, after_docs)) = (before?, after?);
	let diff = build_diff(
		from,
		to,
		(&before_dump, &before_docs),
		(&after_dump, &after_docs),
	);
	let path = paths.diff(&to.version);
	std::fs::create_dir_all(paths.diffs()).map_err(|error| error.to_string())?;
	std::fs::write(&path, to_tab_json(&diff))
		.map_err(|error| format!("{}: {error}", path.display()))?;
	println!(
		"Saved {} API changes and {} description changes from {} to {}.",
		diff.changes.len(),
		diff.descriptions.len(),
		from.version,
		to.version
	);
	Ok(())
}

async fn update_to(
	client: &Client,
	paths: &Paths,
	versions: Vec<RobloxVersion>,
	target: &StudioBuild,
) -> Result<()> {
	let Some(next) = release(client, target).await else {
		println!(
			"{} is out, but its API files are not on the CDN yet. Keeping {}.",
			target.version,
			versions
				.last()
				.map_or("(none)", |version| version.version.as_str())
		);
		return Ok(());
	};
	let mut known = versions;
	if known.is_empty() {
		let builds = parse_studio_builds(&get_json(client, VERSION_HISTORY_URL).await?);
		if let Some(previous) = build_before(&builds, target)
			&& let Some(start) = release(client, previous).await
		{
			known.push(start);
		}
	}
	let previous = known.last().cloned();
	if let Some(previous) = &previous {
		save_diff(client, paths, previous, &next).await?;
	}
	known.push(next.clone());
	write_versions(&paths.versions(), &known).map_err(|error| error.to_string())?;
	println!(
		"Updated {CHANNEL} {} -> {} ({}).",
		previous
			.as_ref()
			.map_or("(none)", |version| version.version.as_str()),
		next.version,
		next.hash
	);
	println!("Run `bun vp run dump` to download the API files.");
	Ok(())
}

pub fn update(paths: &Paths) -> Result<()> {
	runtime()?.block_on(async {
		let client = client()?;
		let build = fetch_live_version(&client).await?;
		let versions = read_versions(&paths.versions()).map_err(|error| error.to_string())?;
		match versions.last() {
			Some(current) if current.hash == build.hash => {
				println!(
					"Already on {CHANNEL} {} ({}).",
					current.version, current.hash
				);
				Ok(())
			}
			Some(current) if compare_versions(&build.version, &current.version).is_le() => {
				println!(
					"{} is not newer than {}. Keeping {}.",
					build.version, current.version, current.version
				);
				Ok(())
			}
			_ => update_to(&client, paths, versions, &build).await,
		}
	})
}

pub fn rediff(paths: &Paths, from: &str, to: &str) -> Result<()> {
	let versions = read_versions(&paths.versions()).map_err(|error| error.to_string())?;
	let find = |version: &str| {
		versions
			.iter()
			.find(|entry| entry.version == version)
			.cloned()
			.ok_or_else(|| format!("roblox-versions.json does not list {version}."))
	};
	let (from, to) = (find(from)?, find(to)?);
	runtime()?.block_on(async { save_diff(&client()?, paths, &from, &to).await })
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn sorts_studio_builds_by_version_and_skips_unknown_upload_ids() {
		let builds = parse_studio_builds(&json!({
			"0.743.0.7431133": "version-4f087c32742644ab",
			"0.744.0.7441077": "version-7aaa6838d36c4448",
			"0.740.19.7400931": "version-c792f79abddd41bd",
			"0.742.0.7421053": "version-9b554450a0fc4e65",
			"0.745.0.7451000": "not-a-version",
		}));
		let versions: Vec<&str> = builds.iter().map(|build| build.version.as_str()).collect();
		assert_eq!(
			versions,
			[
				"0.740.19.7400931",
				"0.742.0.7421053",
				"0.743.0.7431133",
				"0.744.0.7441077"
			]
		);
		assert_eq!(build_before(&builds, &builds[3]), Some(&builds[2]));
		assert_eq!(build_before(&builds, &builds[0]), None);
	}

	#[test]
	fn names_studio_class_icons_by_their_class() {
		assert_eq!(
			icon_class("Shared\\InsertableObjects\\Dark\\Standard\\Part@3x.png"),
			Some("Part")
		);
		assert_eq!(
			icon_class("Shared\\InsertableObjects\\Dark\\Standard\\Part@2x.png"),
			None
		);
		assert_eq!(
			icon_class("Shared\\InsertableObjects\\Light\\Standard\\Part@3x.png"),
			None
		);
	}

	#[test]
	fn reads_upload_hashes() {
		assert_eq!(
			upload_hash("version-7062aec9dbe74b14"),
			Some("7062aec9dbe74b14")
		);
		assert_eq!(upload_hash("version-7062AEC9DBE74B14"), None);
		assert_eq!(upload_hash("7062aec9dbe74b14"), None);
	}
}

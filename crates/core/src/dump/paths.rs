use std::path::{Path, PathBuf};

use super::{RobloxVersion, read_versions};

const VERSIONS_FILE: &str = "roblox-versions.json";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataFile {
	ApiDump,
	ApiDocs,
	StudioIcons,
}

impl DataFile {
	pub const ALL: [Self; 3] = [Self::ApiDump, Self::ApiDocs, Self::StudioIcons];

	pub const fn prefix(self) -> &'static str {
		match self {
			Self::ApiDump => "Full-API-Dump-",
			Self::ApiDocs => "Api-Docs-en-us-",
			Self::StudioIcons => "Studio-Icons-",
		}
	}

	pub fn file_name(self, version: &RobloxVersion) -> String {
		format!("{}{}-{}.json", self.prefix(), version.hash, version.channel)
	}
}

pub struct Paths {
	pub root: PathBuf,
}

impl Paths {
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}

	pub fn locate(dir: &Path) -> Option<Self> {
		dir.ancestors()
			.find(|dir| dir.join(VERSIONS_FILE).is_file())
			.map(Self::new)
	}

	pub fn data(&self) -> PathBuf {
		self.root.join("data")
	}

	pub fn data_file(&self, file: DataFile, version: &RobloxVersion) -> PathBuf {
		self.data().join(file.file_name(version))
	}

	pub fn diffs(&self) -> PathBuf {
		self.root.join("diffs")
	}

	pub fn diff(&self, version: &str) -> PathBuf {
		self.diffs().join(format!("{version}.json"))
	}

	pub fn versions(&self) -> PathBuf {
		self.root.join(VERSIONS_FILE)
	}

	pub fn generated(&self) -> PathBuf {
		self.root.join("src").join("generated")
	}

	pub fn tracked_versions(&self) -> Result<Vec<RobloxVersion>, String> {
		let versions = read_versions(&self.versions()).map_err(|error| error.to_string())?;
		if versions.is_empty() {
			return Err(format!(
				"{VERSIONS_FILE} has no versions. Run `bun vp run dump:update` first."
			));
		}
		Ok(versions)
	}

	pub fn current_version(&self) -> Result<RobloxVersion, String> {
		Ok(self
			.tracked_versions()?
			.pop()
			.expect("tracked versions are never empty"))
	}
}

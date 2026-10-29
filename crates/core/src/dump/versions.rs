use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RobloxVersion {
	pub channel: String,
	pub version: String,
	pub hash: String,
	pub published: String,
}

impl RobloxVersion {
	pub fn dump_file_name(&self) -> String {
		format!("Full-API-Dump-{}-{}.json", self.hash, self.channel)
	}

	pub fn api_docs_file_name(&self) -> String {
		format!("Api-Docs-en-us-{}-{}.json", self.hash, self.channel)
	}

	pub fn studio_icons_file_name(&self) -> String {
		format!("Studio-Icons-{}-{}.json", self.hash, self.channel)
	}
}

pub fn read_versions(path: &Path) -> std::io::Result<Vec<RobloxVersion>> {
	match std::fs::read(path) {
		Ok(bytes) => serde_json::from_slice(&bytes).map_err(std::io::Error::other),
		Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
		Err(error) => Err(error),
	}
}

pub fn write_versions(path: &Path, versions: &[RobloxVersion]) -> std::io::Result<()> {
	std::fs::write(path, super::to_tab_json(&versions))
}

pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
	let parse = |text: &str| -> Vec<f64> {
		text.split('.')
			.map(|part| part.parse::<f64>().unwrap_or(f64::NAN))
			.collect()
	};
	let (left, right) = (parse(a), parse(b));
	for index in 0..left.len().max(right.len()) {
		let difference =
			left.get(index).copied().unwrap_or(0.0) - right.get(index).copied().unwrap_or(0.0);
		if difference > 0.0 {
			return std::cmp::Ordering::Greater;
		}
		if difference < 0.0 {
			return std::cmp::Ordering::Less;
		}
	}
	std::cmp::Ordering::Equal
}

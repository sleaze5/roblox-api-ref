use rapi_routes::upload_name;

pub fn release_name(version: &str) -> String {
	format!("Roblox Studio {version}")
}

pub fn release_text(version: &str, hash: &str) -> String {
	format!("{} ({})", release_name(version), upload_name(hash))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn names_the_studio_version_and_its_upload_hash() {
		assert_eq!(release_name("0.744.0.7441077"), "Roblox Studio 0.744.0.7441077");
		assert_eq!(
			release_text("0.744.0.7441077", "7aaa6838d36c4448"),
			"Roblox Studio 0.744.0.7441077 (version-7aaa6838d36c4448)"
		);
	}
}

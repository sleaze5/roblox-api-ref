use url::Url;

use crate::model::DocLink;

fn site(url: &str) -> String {
	let parsed = Url::parse(url).ok();
	let host = parsed.as_ref().and_then(Url::host_str).unwrap_or(url);
	host.strip_prefix("www.").unwrap_or(host).to_owned()
}

pub fn doc_link_hint(link: &DocLink) -> String {
	match link {
		DocLink::Class { name } => format!("Class: {name}"),
		DocLink::Member {
			owner,
			member,
			name,
		} => format!("{}: {owner}{}{name}", member.name(), member.separator()),
		DocLink::Enum { name } => format!("Enum: {name}"),
		DocLink::EnumItem { owner, name } => format!("Enum item: {owner}.{name}"),
		DocLink::Type { name } => format!("Data type: {name}"),
		DocLink::External { url } => site(url),
	}
}

#[cfg(test)]
mod tests {
	use rapi_routes::MemberKind;

	use super::*;

	#[test]
	fn names_the_kind_and_the_api_of_an_internal_link() {
		assert_eq!(
			doc_link_hint(&DocLink::Class {
				name: "Part".to_owned()
			}),
			"Class: Part"
		);
		assert_eq!(
			doc_link_hint(&DocLink::Member {
				owner: "Instance".to_owned(),
				member: MemberKind::Method,
				name: "Clone".to_owned(),
			}),
			"Method: Instance:Clone"
		);
		assert_eq!(
			doc_link_hint(&DocLink::Member {
				owner: "Instance".to_owned(),
				member: MemberKind::Property,
				name: "Name".to_owned(),
			}),
			"Property: Instance.Name"
		);
		assert_eq!(
			doc_link_hint(&DocLink::Enum {
				name: "KeyCode".to_owned()
			}),
			"Enum: KeyCode"
		);
		assert_eq!(
			doc_link_hint(&DocLink::EnumItem {
				owner: "PreferredInput".to_owned(),
				name: "MicroGamepad".to_owned(),
			}),
			"Enum item: PreferredInput.MicroGamepad"
		);
		assert_eq!(
			doc_link_hint(&DocLink::Type {
				name: "Vector3".to_owned()
			}),
			"Data type: Vector3"
		);
	}

	#[test]
	fn names_the_site_of_an_external_link() {
		let external = |url: &str| {
			doc_link_hint(&DocLink::External {
				url: url.to_owned(),
			})
		};
		assert_eq!(
			external("https://create.roblox.com/docs/projects/teleport"),
			"create.roblox.com"
		);
		assert_eq!(
			external("https://www.roblox.com/premium/membership"),
			"roblox.com"
		);
		assert_eq!(
			external("https://en.wikipedia.org/wiki/MD5"),
			"en.wikipedia.org"
		);
	}
}

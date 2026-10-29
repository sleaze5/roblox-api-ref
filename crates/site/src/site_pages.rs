//! The About, Contact, and Privacy pages.

use rapi_core::api::release::release_text;
use rapi_core::icons::Icon;
use rapi_routes::{ABOUT, CLASSES, CONTACT, ENUMS, LLMS_TXT, PRIVACY, UPDATES};

use crate::meta::{COMMITS_URL, ISSUES_URL, REPOSITORY_URL, SITE_DESCRIPTION, SchemaType};
use crate::routes::SitePageName;

pub enum Inline {
	Text(String),
	Link {
		text: &'static str,
		href: &'static str,
	},
	Code(&'static str),
}

pub enum Block {
	Paragraph(Vec<Inline>),
	List(Vec<Vec<Inline>>),
}

pub struct ContentSection {
	pub id: &'static str,
	pub heading: &'static str,
	pub blocks: Vec<Block>,
}

pub struct PageContent {
	pub lead: &'static str,
	pub sections: Vec<ContentSection>,
}

pub struct SitePage {
	pub title: &'static str,
	pub href: &'static str,
	pub icon: Icon,
	pub schema_type: SchemaType,
}

pub const fn site_page(name: SitePageName) -> SitePage {
	match name {
		SitePageName::About => SitePage {
			title: "About",
			href: ABOUT,
			icon: Icon::Info,
			schema_type: SchemaType::AboutPage,
		},
		SitePageName::Contact => SitePage {
			title: "Contact",
			href: CONTACT,
			icon: Icon::EnvelopeSimple,
			schema_type: SchemaType::ContactPage,
		},
		SitePageName::Privacy => SitePage {
			title: "Privacy policy",
			href: PRIVACY,
			icon: Icon::LockSimple,
			schema_type: SchemaType::WebPage,
		},
	}
}

pub fn is_external(href: &str) -> bool {
	href.starts_with("http:") || href.starts_with("https:")
}

fn text(value: &str) -> Inline {
	Inline::Text(value.to_owned())
}

const fn link(text: &'static str, href: &'static str) -> Inline {
	Inline::Link { text, href }
}

fn section(id: &'static str, heading: &'static str, blocks: Vec<Block>) -> ContentSection {
	ContentSection {
		id,
		heading,
		blocks,
	}
}

pub fn page_content(name: SitePageName, version: &str, hash: &str) -> PageContent {
	match name {
		SitePageName::About => about(version, hash),
		SitePageName::Contact => contact(),
		SitePageName::Privacy => privacy(),
	}
}

fn about(version: &str, hash: &str) -> PageContent {
	PageContent {
		lead: SITE_DESCRIPTION,
		sections: vec![
			section(
				"what-it-covers",
				"What it covers",
				vec![
					Block::Paragraph(vec![
						text("The site lists every "),
						link("class", CLASSES),
						text(", member, "),
						link("enum", ENUMS),
						text(
							", enum item, and data type in the Roblox Engine API. Each member shows its Luau signature, its default value, and the security level, thread safety, capabilities, and tags that control where scripts can use it. The site also shows:",
						),
					]),
					Block::List(vec![
						vec![text(
							"The members that each class inherits from its superclasses.",
						)],
						vec![text(
							"The properties, parameters, and return values that use each data type and enum.",
						)],
						vec![
							text("The API changes between Roblox Studio builds, on the "),
							link("Updates", UPDATES),
							text(" page."),
						],
					]),
				],
			),
			section(
				"data-sources",
				"Where the data comes from",
				vec![
					Block::Paragraph(vec![text(
						"Every page is generated from two files that Roblox publishes with each Roblox Studio build:",
					)]),
					Block::List(vec![
						vec![text(
							"The API dump, which lists the classes, members, and enums.",
						)],
						vec![text("The English API docs, which hold the descriptions.")],
					]),
					Block::Paragraph(vec![Inline::Text(format!(
						"Descriptions are copied from the API docs without changes. This build covers {}. Every hour, the site checks the LIVE channel for a new Roblox Studio build, and rebuilds when Roblox publishes one.",
						release_text(version, hash)
					))]),
				],
			),
			section(
				"agents",
				"For AI agents and tools",
				vec![Block::Paragraph(vec![
					text(
						"Every page has a Markdown version. To get it, remove the trailing slash from the page URL and add ",
					),
					Inline::Code(".md"),
					text(", or request the page URL with "),
					Inline::Code("Accept: text/markdown"),
					text(". The "),
					link("llms.txt", LLMS_TXT),
					text(" file lists the URL patterns and indexes."),
				])],
			),
			section(
				"affiliation",
				"Not affiliated with Roblox",
				vec![Block::Paragraph(vec![text(
					"This site is not an official Roblox product. It is not affiliated with or endorsed by Roblox Corporation. Roblox is a trademark of Roblox Corporation.",
				)])],
			),
			section(
				"source-code",
				"Source code",
				vec![Block::Paragraph(vec![
					text(
						"The site is open source. The code, the build history, and the change list for each update are ",
					),
					link("on GitHub", REPOSITORY_URL),
					text(". To report a problem, see "),
					link("Contact", CONTACT),
					text("."),
				])],
			),
		],
	}
}

fn contact() -> PageContent {
	PageContent {
		lead: "Report a problem with the site or suggest a feature.",
		sections: vec![
			section(
				"report",
				"Report a problem",
				vec![Block::Paragraph(vec![
					text(
						"To report wrong or missing data, a broken page or link, or a problem with search or filters, ",
					),
					link("open an issue on GitHub", ISSUES_URL),
					text(
						". Include the page URL, what you expected, and what you saw. Issues are public, so do not include personal information.",
					),
				])],
			),
			section(
				"suggest",
				"Suggest a feature",
				vec![Block::Paragraph(vec![
					text("To suggest a feature, such as a new filter or search keyword, "),
					link("open an issue on GitHub", ISSUES_URL),
					text(". Describe what you want to do with the feature."),
				])],
			),
			section(
				"source",
				"Source code",
				vec![Block::Paragraph(vec![
					text("The code and the history of every change are in the "),
					link("GitHub repository", REPOSITORY_URL),
					text("."),
				])],
			),
			section(
				"privacy",
				"Privacy",
				vec![Block::Paragraph(vec![
					text("The "),
					link("privacy policy", PRIVACY),
					text(
						" explains what the site collects. To ask about it, open an issue on GitHub.",
					),
				])],
			),
		],
	}
}

const CLOUDFLARE_PRIVACY_URL: &str = "https://www.cloudflare.com/privacypolicy/";

fn privacy() -> PageContent {
	PageContent {
		lead: "This site does not collect personal information, set cookies, or track you.",
		sections: vec![
			section(
				"collection",
				"What the site collects",
				vec![
					Block::Paragraph(vec![text(
						"Nothing. Every page is a static file. The site has none of the following:",
					)]),
					Block::List(vec![
						vec![text("Accounts, sign-in, forms, or comments.")],
						vec![text("Analytics, advertising, or tracking scripts.")],
						vec![text(
							"Cookies, local storage, or other data saved in your browser.",
						)],
					]),
				],
			),
			section(
				"search",
				"Search and filters",
				vec![
					Block::Paragraph(vec![text(
						"Search runs in your browser. The site downloads its search index from this domain and matches your query on your device. Your searches are not sent to a server as you type.",
					)]),
					Block::Paragraph(vec![text(
						"Search queries and filters are saved in the page URL, so that you can share or bookmark a view. When you load that URL, the query is sent to the host as part of the address.",
					)]),
				],
			),
			section(
				"hosting",
				"Hosting",
				vec![Block::Paragraph(vec![
					text(
						"The site is hosted on Cloudflare Pages. Cloudflare delivers every page through its network and processes request data, such as your IP address, user agent, and the requested URL, to serve the site and protect it from abuse, as described in ",
					),
					link("the Cloudflare privacy policy", CLOUDFLARE_PRIVACY_URL),
					text(
						". Cloudflare may show the site owner aggregate traffic statistics, such as request counts. The site does not identify or profile visitors.",
					),
				])],
			),
			section(
				"third-parties",
				"Third parties",
				vec![Block::Paragraph(vec![text(
					"Pages, fonts, scripts, and the search index all load from this domain. The site makes no requests to other services. Links to other sites open in a new tab and do not tell those sites which page you came from.",
				)])],
			),
			section(
				"changes",
				"Changes",
				vec![Block::Paragraph(vec![
					text("When this policy changes, this page is updated, and "),
					link("the source history on GitHub", COMMITS_URL),
					text(" records the change. For questions, see "),
					link("Contact", CONTACT),
					text("."),
				])],
			),
		],
	}
}

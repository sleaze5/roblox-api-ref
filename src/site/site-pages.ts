import type { Release } from "../api/release.ts"
import { aboutHref, contactHref, privacyHref } from "../api/routes.ts"
import type { IconName } from "../components/icon-names.ts"
import type { SchemaType } from "../server/page-module.ts"
import { aboutContent } from "./content/about.ts"
import { contactContent } from "./content/contact.ts"
import type { PageContent } from "./content/page-content.ts"
import { privacyContent } from "./content/privacy.ts"

export type SitePageName = "about" | "contact" | "privacy"

export type SitePage = {
	title: string
	href: string
	icon: IconName
	schemaType: SchemaType
	content: (release: Release) => PageContent
}

export const sitePages: Record<SitePageName, SitePage> = {
	about: {
		title: "About",
		href: aboutHref(),
		icon: "info",
		schemaType: "AboutPage",
		content: aboutContent,
	},
	contact: {
		title: "Contact",
		href: contactHref(),
		icon: "envelope-simple",
		schemaType: "ContactPage",
		content: contactContent,
	},
	privacy: {
		title: "Privacy policy",
		href: privacyHref(),
		icon: "lock-simple",
		schemaType: "WebPage",
		content: privacyContent,
	},
}

import { absoluteUrl } from "../src/site/origin.ts"
import { repositoryUrl } from "../src/site/repository.ts"
import { siteDescription, siteName } from "../src/site/brand.ts"

type Node = Record<string, unknown>

export const organizationId = absoluteUrl("/#organization")

export function organization(): Node {
	return {
		"@type": "Organization",
		"@id": organizationId,
		"name": siteName,
		"url": absoluteUrl("/"),
		"logo": absoluteUrl("/og-image.png"),
		"description": siteDescription,
		"sameAs": [repositoryUrl],
		"contactPoint": {
			"@type": "ContactPoint",
			"contactType": "customer support",
			"url": absoluteUrl("/contact/"),
			"availableLanguage": "en",
		},
	}
}

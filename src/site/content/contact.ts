import { privacyHref } from "../../api/routes.ts"
import { issuesUrl, repositoryUrl } from "../repository.ts"
import { paragraph, type PageContent } from "./page-content.ts"

export function contactContent(): PageContent {
	return {
		lead: "Report a problem with the site or suggest a feature.",
		sections: [
			{
				id: "report",
				heading: "Report a problem",
				blocks: [
					paragraph(
						"To report wrong or missing data, a broken page or link, or a problem with search or filters, ",
						{ text: "open an issue on GitHub", href: issuesUrl },
						". Include the page URL, what you expected, and what you saw. Issues are public, so do not include personal information.",
					),
				],
			},
			{
				id: "suggest",
				heading: "Suggest a feature",
				blocks: [
					paragraph(
						"To suggest a feature, such as a new filter or search keyword, ",
						{ text: "open an issue on GitHub", href: issuesUrl },
						". Describe what you want to do with the feature.",
					),
				],
			},
			{
				id: "source",
				heading: "Source code",
				blocks: [
					paragraph(
						"The code and the history of every change are in the ",
						{ text: "GitHub repository", href: repositoryUrl },
						".",
					),
				],
			},
			{
				id: "privacy",
				heading: "Privacy",
				blocks: [
					paragraph(
						"The ",
						{ text: "privacy policy", href: privacyHref() },
						" explains what the site collects. To ask about it, open an issue on GitHub.",
					),
				],
			},
		],
	}
}

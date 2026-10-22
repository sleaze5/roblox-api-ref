import { contactHref } from "../../api/routes.ts"
import { commitsUrl } from "../repository.ts"
import { list, paragraph, type PageContent } from "./page-content.ts"

const pagesDataUrl =
	"https://docs.github.com/en/pages/getting-started-with-github-pages/about-github-pages#data-collection"

export function privacyContent(): PageContent {
	return {
		lead: "This site does not collect personal information, set cookies, or track you.",
		sections: [
			{
				id: "collection",
				heading: "What the site collects",
				blocks: [
					paragraph(
						"Nothing. Every page is a static file. The site has none of the following:",
					),
					list(
						["Accounts, sign-in, forms, or comments."],
						["Analytics, advertising, or tracking scripts."],
						[
							"Cookies, local storage, or other data saved in your browser.",
						],
					),
				],
			},
			{
				id: "search",
				heading: "Search and filters",
				blocks: [
					paragraph(
						"Search runs in your browser. The site downloads its search index from this domain and matches your query on your device. Your searches are not sent to a server as you type.",
					),
					paragraph(
						"Search queries and filters are saved in the page URL, so that you can share or bookmark a view. When you load that URL, the query is sent to the host as part of the address.",
					),
				],
			},
			{
				id: "hosting",
				heading: "Hosting",
				blocks: [
					paragraph(
						"The site is hosted on GitHub Pages. GitHub logs and stores the IP address of each visitor for security purposes, as described in ",
						{ text: "the GitHub Pages documentation", href: pagesDataUrl },
						". This site does not receive these logs, and does not identify or profile visitors.",
					),
				],
			},
			{
				id: "third-parties",
				heading: "Third parties",
				blocks: [
					paragraph(
						"Pages, fonts, scripts, and the search index all load from this domain. The site makes no requests to other services. Links to other sites open in a new tab and do not tell those sites which page you came from.",
					),
				],
			},
			{
				id: "changes",
				heading: "Changes",
				blocks: [
					paragraph(
						"When this policy changes, this page is updated, and ",
						{ text: "the source history on GitHub", href: commitsUrl },
						" records the change. For questions, see ",
						{ text: "Contact", href: contactHref() },
						".",
					),
				],
			},
		],
	}
}

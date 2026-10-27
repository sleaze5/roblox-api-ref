import type { Breadcrumb } from "./breadcrumbs.ts"
import type { MarkdownPage } from "./markdown/page-markdown.ts"

export type SchemaType =
	| "WebSite"
	| "CollectionPage"
	| "TechArticle"
	| "AboutPage"
	| "ContactPage"
	| "WebPage"

export type RenderedPage = {
	status: number
	title: string
	body: string
	schemaType: SchemaType | undefined
	breadcrumbs: Breadcrumb[]
	markdown: MarkdownPage
	modified: string
}

export type PageModule = {
	routes: () => string[]
	renderPage: (path: string) => RenderedPage
	lastModified: () => string
	feed: () => string
}

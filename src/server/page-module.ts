import type { Breadcrumb } from "./breadcrumbs.ts"
import type { MarkdownPage } from "./markdown/page-markdown.ts"

export type SchemaType = "WebSite" | "CollectionPage" | "TechArticle"

export type RenderedPage = {
	status: number
	title: string
	description: string
	body: string
	schemaType: SchemaType | undefined
	breadcrumbs: Breadcrumb[]
	markdown: MarkdownPage | undefined
	modified: string
}

export type PageModule = {
	routes: () => string[]
	renderPage: (path: string) => RenderedPage
	lastModified: () => string
	llmsTxt: () => string
}

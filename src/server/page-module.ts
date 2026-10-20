export type RenderedPage = {
	status: number
	title: string
	description: string
	body: string
}

export type PageModule = {
	routes: () => string[]
	renderPage: (path: string) => RenderedPage
}

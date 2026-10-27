import { renderToString, type JSX } from "@solidjs/web"
import type { Api } from "../api/types.ts"
import { ClassPage } from "../pages/class/ClassPage.tsx"
import { EnumPage } from "../pages/enum/EnumPage.tsx"
import { FacetPage } from "../pages/facet/FacetPage.tsx"
import { HomePage } from "../pages/HomePage.tsx"
import { ClassIndexPage } from "../pages/index/ClassIndexPage.tsx"
import { EnumIndexPage } from "../pages/index/EnumIndexPage.tsx"
import { IndexPage } from "../pages/index/IndexPage.tsx"
import { RemovedIndexPage } from "../pages/index/RemovedIndexPage.tsx"
import { TypeIndexPage } from "../pages/index/TypeIndexPage.tsx"
import { FacetIndexPage } from "../pages/index/FacetIndexPage.tsx"
import { NotFoundPage } from "../pages/NotFoundPage.tsx"
import { SitePage } from "../pages/site/SitePage.tsx"
import { TypePage } from "../pages/type/TypePage.tsx"
import { UpdatePage } from "../pages/updates/UpdatePage.tsx"
import { UpdatesPage } from "../pages/updates/UpdatesPage.tsx"
import { sitePages } from "../site/site-pages.ts"
import { breadcrumbs } from "./breadcrumbs.ts"
import { updatesFeed } from "./feed.ts"
import { loadApi } from "./load-api.ts"
import { loadUpdates } from "./load-updates.ts"
import { pageMarkdown } from "./markdown/page-markdown.ts"
import { matchRoute, routePaths, type IndexList, type Route } from "./match-route.ts"
import type { RenderedPage } from "./page-module.ts"
import { pageTitle } from "./page-title.ts"
import { schemaType } from "./schema-type.ts"

const indexListPages: Record<IndexList, (api: Api) => JSX.Element> = {
	types: (api) => <TypeIndexPage api={api} />,
	removed: (api) => <RemovedIndexPage api={api} />,
}

function Page(props: { route: Route }) {
	const api = loadApi()
	const updates = loadUpdates()
	switch (props.route.page) {
		case "home":
			return <HomePage api={api} updates={updates} />
		case "classes":
			return <ClassIndexPage api={api} />
		case "enums":
			return <EnumIndexPage api={api} />
		case "index":
			return <IndexPage api={api} />
		case "index-list":
			return indexListPages[props.route.list](api)
		case "facet-index":
			return <FacetIndexPage api={api} kind={props.route.kind} />
		case "class":
			return <ClassPage api={api} item={props.route.item} />
		case "enum":
			return <EnumPage api={api} item={props.route.item} />
		case "type":
			return <TypePage api={api} item={props.route.item} />
		case "facet":
			return <FacetPage api={api} facet={props.route.facet} />
		case "updates":
			return <UpdatesPage api={api} updates={updates} />
		case "update":
			return <UpdatePage api={api} updates={updates} item={props.route.item} />
		case "site":
			return <SitePage release={api} page={sitePages[props.route.name]} />
		case "not-found":
			return <NotFoundPage api={api} />
	}
}

export function routes(): string[] {
	return routePaths(loadApi(), loadUpdates())
}

export function renderPage(path: string): RenderedPage {
	const api = loadApi()
	const updates = loadUpdates()
	const route = matchRoute(api, updates, path)
	return {
		status: route.page === "not-found" ? 404 : 200,
		title: pageTitle(route),
		body: renderToString(() => <Page route={route} />),
		schemaType: schemaType(route),
		breadcrumbs: breadcrumbs(route),
		markdown: pageMarkdown(api, updates, route),
		modified: api.published,
	}
}

export function lastModified(): string {
	return loadApi().published
}

export function feed(): string {
	return updatesFeed(loadUpdates())
}

import { renderToString } from "@solidjs/web"
import { ClassPage } from "../pages/class/ClassPage.tsx"
import { EnumPage } from "../pages/enum/EnumPage.tsx"
import { FacetPage } from "../pages/facet/FacetPage.tsx"
import { HomePage } from "../pages/HomePage.tsx"
import { ClassIndexPage } from "../pages/index/ClassIndexPage.tsx"
import { EnumIndexPage } from "../pages/index/EnumIndexPage.tsx"
import { IndexPage } from "../pages/index/IndexPage.tsx"
import { TypeIndexPage } from "../pages/index/TypeIndexPage.tsx"
import { FacetIndexPage } from "../pages/index/FacetIndexPage.tsx"
import { NotFoundPage } from "../pages/NotFoundPage.tsx"
import { TypePage } from "../pages/type/TypePage.tsx"
import { describeRoute } from "./describe-route.ts"
import { loadApi } from "./load-api.ts"
import { matchRoute, routePaths, type Route } from "./match-route.ts"
import type { RenderedPage } from "./page-module.ts"

function Page(props: { route: Route }) {
	const api = loadApi()
	switch (props.route.page) {
		case "home":
			return <HomePage api={api} />
		case "classes":
			return <ClassIndexPage api={api} />
		case "enums":
			return <EnumIndexPage api={api} />
		case "index":
			return <IndexPage api={api} />
		case "types":
			return <TypeIndexPage api={api} />
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
		case "not-found":
			return <NotFoundPage api={api} />
	}
}

export function routes(): string[] {
	return routePaths(loadApi())
}

export function renderPage(path: string): RenderedPage {
	const api = loadApi()
	const route = matchRoute(api, path)
	return {
		status: route.page === "not-found" ? 404 : 200,
		...describeRoute(route, api.version),
		body: renderToString(() => <Page route={route} />),
	}
}

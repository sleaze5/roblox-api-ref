import type { JSX } from "@solidjs/web"
import { DrawerSections } from "./DrawerSections.tsx"
import { Footer } from "./Footer.tsx"
import type { Release } from "../api/release.ts"
import { Header } from "./Header.tsx"
import { SidebarScroll } from "./SectionSidebar.tsx"
import type { Section } from "./section-links.ts"

const drawerClass =
	"fixed inset-y-0 left-0 m-0 motion-drawer h-dvh w-sidebar max-w-drawer overflow-x-hidden overflow-y-auto overscroll-contain border-r border-line bg-surface p-3 text-sm text-fg"

export function Shell(props: {
	section?: Section
	release: Release
	sidebar?: JSX.Element
	rail?: JSX.Element
	children: JSX.Element
}) {
	const hasSidebar = props.sidebar !== undefined
	return (
		<>
			<a
				href="#main"
				class="sr-only rounded-sm bg-raised px-3 py-2 text-fg focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-30">
				Skip to content
			</a>
			<Header section={props.section} release={props.release} />
			<div class="flex">
				<nav
					id="sidebar"
					popover="auto"
					aria-label="Navigation"
					data-drawer-only={hasSidebar ? undefined : ""}
					class={[
						drawerClass,
						hasSidebar
							? "md:sticky md:top-header md:h-below-header md:max-w-none md:shrink-0"
							: undefined,
					]}>
					<DrawerSections section={props.section} />
					{props.sidebar}
				</nav>
				{hasSidebar ? <SidebarScroll /> : null}
				{props.rail === undefined ? null : (
					<aside class="fixed top-header right-0 hidden h-below-header w-rail overflow-y-auto overscroll-contain py-8 pr-4 lg:block">
						{props.rail}
					</aside>
				)}
				<div
					class={[
						"flex min-h-below-header min-w-0 flex-1 flex-col",
						props.rail === undefined ? undefined : "lg:rail-gutter",
					]}>
					<main id="main" class="flex-1 px-4 py-6 md:px-8 md:py-8">
						<div class="mx-auto max-w-content">{props.children}</div>
					</main>
					<Footer />
				</div>
			</div>
			<div data-search-root="" />
			<div id="tooltip" popover="manual" aria-hidden="true" class="tooltip" />
			<div id="announcer" aria-live="polite" class="sr-only" />
		</>
	)
}

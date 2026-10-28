import { compareNames } from "../api/compare-names.ts"
import {
	classesHref,
	classHref,
	enumsHref,
	indexHref,
	statsHref,
	updatesHref,
} from "../api/routes.ts"
import { uploadName } from "../api/release.ts"
import type { Api } from "../api/types.ts"
import type { Update } from "../api/update-types.ts"
import type { IconName } from "../components/icon-names.ts"
import { Icon } from "../components/Icon.tsx"
import { Shell } from "../layout/Shell.tsx"
import { siteSummary } from "../site/brand.ts"

const examples = [
	"Part.Touched",
	"TweenService.",
	"is:enum Material",
	"tag:yields data",
	"type:Color3",
]

function BrowseLink(props: {
	href: string
	label: string
	count?: number
	icon: IconName
}) {
	return (
		<a
			href={props.href}
			class="flex items-center gap-3 rounded-md border border-line px-4 py-3 motion-colors hover:bg-hover">
			<Icon name={props.icon} size={20} class="text-fg-muted" />
			<span class="font-medium">{props.label}</span>
			{props.count === undefined ? null : (
				<span class="ml-auto text-fg-faint">{props.count}</span>
			)}
		</a>
	)
}

export function HomePage(props: { api: Api; updates: Update[] }) {
	const classCount = Object.keys(props.api.classes).length
	const enumCount = Object.keys(props.api.enums).length
	const services = Object.values(props.api.classes)
		.filter(
			(item) =>
				item.tags.includes("Service") && !item.tags.includes("Deprecated"),
		)
		.sort((a, b) => compareNames(a.name, b.name))
	return (
		<Shell release={props.api}>
			<div class="flex flex-col gap-12">
				<section class="flex flex-col gap-4 pt-8">
					<div class="flex items-center gap-3">
						<h1 class="text-xl font-semibold text-fg-bright">
							Roblox API Reference
						</h1>
						<span class="tag-chip">Unofficial</span>
					</div>
					<p class="max-w-prose text-base text-fg-muted">
						{siteSummary} This build covers {classCount} classes and{" "}
						{enumCount} enums from Roblox Studio{" "}
						<span class="font-mono text-sm text-fg">
							{props.api.version}
						</span>{" "}
						(
						<span class="font-mono text-sm text-fg">
							{uploadName(props.api.hash)}
						</span>
						), with every default value in the API dump.
					</p>
					<button
						type="button"
						data-search-open=""
						class="flex h-12 w-full min-w-0 items-center gap-3 rounded-md border border-line bg-raised px-4 text-left text-base text-fg-faint motion-colors hover:border-line-strong hover:text-fg-muted">
						<Icon name="magnifying-glass" size={20} />
						<span class="truncate">
							Search classes, members, enums, and data types
						</span>
						<kbd class="ml-auto hidden kbd md:block">/</kbd>
					</button>
					<ul class="flex flex-wrap gap-2" aria-label="Example searches">
						{examples.map((example) => (
							<li>
								<button
									type="button"
									data-search-open={example}
									aria-label={`Search for ${example}`}
									class="button font-mono font-normal">
									{example}
								</button>
							</li>
						))}
					</ul>
				</section>
				<section class="flex flex-col gap-3" aria-labelledby="browse-title">
					<h2 id="browse-title" class="text-lg font-semibold">
						Browse
					</h2>
					<div class="grid grid-cols-1 gap-3 md:grid-cols-2">
						<BrowseLink
							href={classesHref()}
							label="Class tree"
							count={classCount}
							icon="tree-structure"
						/>
						<BrowseLink
							href={enumsHref()}
							label="Enums"
							count={enumCount}
							icon="list-bullets"
						/>
						<BrowseLink href={indexHref()} label="Index" icon="books" />
						<BrowseLink
							href={updatesHref()}
							label="Updates"
							count={props.updates.length}
							icon="git-diff"
						/>
						<BrowseLink
							href={statsHref()}
							label="Statistics"
							icon="chart-bar"
						/>
					</div>
				</section>
				<section class="flex flex-col gap-3" aria-labelledby="services-title">
					<h2 id="services-title" class="text-lg font-semibold">
						Services
					</h2>
					<ul class="grid grid-cols-1 gap-x-4 md:grid-cols-2 lg:grid-cols-3">
						{services.map((item) => (
							<li>
								<a
									href={classHref(item.name)}
									class="flex h-7 items-center rounded-sm px-2 font-mono text-code-type motion-colors hover:bg-hover">
									<span class="truncate">{item.name}</span>
								</a>
							</li>
						))}
					</ul>
				</section>
			</div>
		</Shell>
	)
}

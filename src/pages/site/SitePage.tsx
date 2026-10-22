import type { Release } from "../../api/release.ts"
import { rootCrumbs } from "../../components/page/crumbs.ts"
import { PageHeader } from "../../components/page/PageHeader.tsx"
import { PageLead } from "../../components/page/PageLead.tsx"
import { Shell } from "../../layout/Shell.tsx"
import type { Block, ContentSection } from "../../site/content/page-content.ts"
import type { SitePage as SitePageInfo } from "../../site/site-pages.ts"
import { InlineContent } from "./InlineContent.tsx"

function ContentBlock(props: { block: Block }) {
	const block = props.block
	if (block.kind === "paragraph") {
		return (
			<p>
				<InlineContent content={block.content} />
			</p>
		)
	}
	return (
		<ul>
			{block.items.map((item) => (
				<li>
					<InlineContent content={item} />
				</li>
			))}
		</ul>
	)
}

function Section(props: { section: ContentSection }) {
	const titleId = `${props.section.id}-title`
	return (
		<section class="flex flex-col gap-3" aria-labelledby={titleId}>
			<h2 id={titleId} class="text-lg font-semibold">
				{props.section.heading}
			</h2>
			<div class="doc-text flex max-w-prose flex-col gap-3 text-base text-fg">
				{props.section.blocks.map((block) => (
					<ContentBlock block={block} />
				))}
			</div>
		</section>
	)
}

export function SitePage(props: { release: Release; page: SitePageInfo }) {
	const content = props.page.content(props.release)
	return (
		<Shell release={props.release}>
			<div class="flex flex-col gap-10">
				<PageHeader
					crumbs={rootCrumbs}
					icon={props.page.icon}
					title={props.page.title}>
					<PageLead>{content.lead}</PageLead>
				</PageHeader>
				{content.sections.map((section) => (
					<Section section={section} />
				))}
			</div>
		</Shell>
	)
}

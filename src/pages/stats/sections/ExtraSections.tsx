import { capabilitiesLead, tagsLead } from "../../../api/stats-text.ts"
import type { Stats } from "../../../api/stats-types.ts"
import type { Api } from "../../../api/types.ts"
import { AccessSection } from "./AccessSection.tsx"
import { DocsSection } from "./DocsSection.tsx"
import { GrowthSection } from "./GrowthSection.tsx"
import { TagSection } from "./TagSection.tsx"

export const extraSections = [
	{ id: "access", label: "Access" },
	{ id: "tags", label: "Tags" },
	{ id: "capabilities", label: "Capabilities" },
	{ id: "descriptions", label: "Descriptions" },
	{ id: "growth", label: "Growth by class" },
] as const

export function ExtraSections(props: { api: Api; stats: Stats }) {
	return (
		<>
			<AccessSection access={props.stats.access} />
			<TagSection
				api={props.api}
				id="tags"
				title="Tags"
				lead={tagsLead}
				kind="tag"
				label="Tag"
				rows={props.stats.tags}
			/>
			<TagSection
				api={props.api}
				id="capabilities"
				title="Capabilities"
				lead={capabilitiesLead}
				kind="capability"
				label="Capability"
				rows={props.stats.capabilities}
			/>
			<DocsSection docs={props.stats.docs} />
			<GrowthSection api={props.api} growth={props.stats.growth} />
		</>
	)
}

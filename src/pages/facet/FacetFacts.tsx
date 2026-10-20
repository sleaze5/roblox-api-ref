import { facetLabels } from "../../api/facets/facet-info.ts"
import { identitiesAllowing } from "../../api/security-access.ts"
import { plainTags } from "../../api/tags.ts"
import { securityLevels, type Facet, type Security } from "../../api/types.ts"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"
import { TagList } from "../../components/TagList.tsx"

function isSecurity(name: string): name is Security {
	return (securityLevels as readonly string[]).includes(name)
}

function KindFacts(props: { facet: Facet }) {
	if (props.facet.kind === "tag") {
		return (
			<Fact label="Shown as" hint="How this tag appears on this site.">
				<TagList tags={plainTags([props.facet.name])} />
			</Fact>
		)
	}
	if (props.facet.kind === "security" && isSecurity(props.facet.name)) {
		return (
			<Fact
				label="Usable by identities"
				hint="Script identities that can use members with this security level.">
				{identitiesAllowing(props.facet.name).join(", ")}
			</Fact>
		)
	}
	return null
}

export function FacetFacts(props: { facet: Facet }) {
	return (
		<FactList>
			<Fact label="Kind" hint="The Index group that this page belongs to.">
				{facetLabels[props.facet.kind]}
			</Fact>
			<Fact
				label="Uses"
				hint={`API items that have this ${facetLabels[props.facet.kind].toLowerCase()}.`}>
				{props.facet.uses.length}
			</Fact>
			<KindFacts facet={props.facet} />
		</FactList>
	)
}

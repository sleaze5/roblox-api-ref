import {
	accessTierHints,
	accessTierLabels,
	accessTiers,
	tierCounts,
} from "../../api/access-tiers.ts"
import type { Member } from "../../api/types.ts"
import { Fact } from "../../components/facts/Fact.tsx"
import { FactList } from "../../components/facts/FactList.tsx"

export function AccessFacts(props: { members: Member[] }) {
	const counts = tierCounts(props.members)
	return props.members.length === 0 ? null : (
		<FactList>
			{accessTiers.map((tier) => (
				<Fact
					label={accessTierLabels[tier]}
					hint={`${accessTierHints[tier]} Counts the members this class defines itself.`}>
					{counts[tier]}
				</Fact>
			))}
		</FactList>
	)
}

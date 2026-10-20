import { typeTokens } from "../../../api/luau-type.ts"
import { isNothing, parameterTokens } from "../../../api/signature.ts"
import type { Member } from "../../../api/types.ts"
import { DetailTable, type DetailRow } from "./DetailTable.tsx"

function parameterRows(member: Member): DetailRow[] {
	return member.kind === "Property"
		? []
		: member.parameters.map((parameter) => ({
				code: parameterTokens(parameter),
				description: parameter.description,
			}))
}

function returnRows(member: Member): DetailRow[] {
	if (member.kind !== "Method" && member.kind !== "Callback") {
		return []
	}
	if (isNothing(member.returns) && member.returnDescription === undefined) {
		return []
	}
	return [{ code: typeTokens(member.returns), description: member.returnDescription }]
}

export function MemberDetails(props: { member: Member }) {
	return (
		<>
			<DetailTable
				caption="Parameters"
				heading="Parameter"
				rows={parameterRows(props.member)}
			/>
			<DetailTable
				caption="Returns"
				heading="Type"
				rows={returnRows(props.member)}
			/>
		</>
	)
}

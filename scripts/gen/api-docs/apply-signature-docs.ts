import type { Member } from "../../../src/api/types.ts"
import type { ApiDocsEntry } from "./api-docs-types.ts"
import type { Describe } from "./describer.ts"

export function applySignatureDocs(
	member: Member,
	entry: ApiDocsEntry | undefined,
	describe: Describe,
): void {
	if (member.kind !== "Method" && member.kind !== "Callback") {
		return
	}
	const params = new Map(
		(entry?.params ?? []).map((param) => [param.name, param.documentation]),
	)
	for (const parameter of member.parameters) {
		const description = describe(params.get(parameter.name))
		if (description !== undefined) {
			parameter.description = description
		}
	}
	const returnDescription = describe(entry?.returns?.[0])
	if (returnDescription !== undefined) {
		member.returnDescription = returnDescription
	}
}

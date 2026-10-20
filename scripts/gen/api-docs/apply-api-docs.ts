import type { Doc } from "../../../src/api/doc-types.ts"
import type { ApiDocs } from "./api-docs-types.ts"
import { applySignatureDocs } from "./apply-signature-docs.ts"
import { describer } from "./describer.ts"
import type { ApiNames } from "./doc/api-names.ts"

function withDescription(target: { description?: Doc }, description: Doc | undefined) {
	if (description !== undefined) {
		target.description = description
	}
}

export function applyApiDocs(names: ApiNames, docs: ApiDocs): void {
	for (const item of Object.values(names.classes)) {
		const describe = describer(names, docs, { kind: "class", name: item.name })
		withDescription(item, describe(`@roblox/globaltype/${item.name}`))
		for (const member of item.members) {
			const key = `@roblox/globaltype/${item.name}.${member.name}`
			withDescription(member, describe(key))
			applySignatureDocs(member, docs[key], describe)
		}
	}
	for (const item of Object.values(names.enums)) {
		const describe = describer(names, docs, { kind: "enum", name: item.name })
		withDescription(item, describe(`@roblox/global/Enum.${item.name}`))
		for (const enumItem of item.items) {
			withDescription(
				enumItem,
				describe(`@roblox/enum/${item.name}.${enumItem.name}`),
			)
		}
	}
}

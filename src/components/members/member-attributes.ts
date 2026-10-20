import { isDeprecated } from "../../api/members.ts"
import { memberTagKeys } from "../../api/member-filter-tags.ts"
import type { Member } from "../../api/types.ts"

export function memberAttributes(member: Member): Record<string, string | undefined> {
	return {
		"data-member": "",
		"data-kind": member.kind,
		"data-name": member.name,
		"data-deprecated": isDeprecated(member) ? "" : undefined,
		"data-tags": memberTagKeys(member).join(" "),
	}
}

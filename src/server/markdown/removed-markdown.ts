import type { RemovedOwner } from "../../api/item-history.ts"
import { removedLead } from "../../api/removed-text.ts"
import { formatDate } from "../../api/update-text.ts"
import type { Api } from "../../api/types.ts"
import { joinBlocks } from "./document.ts"
import { inlineCode } from "./escape.ts"

function lines(title: string, owners: RemovedOwner[]): string {
	return owners.length === 0
		? `## ${title}\n\nNone.`
		: `## ${title}\n\n${owners
				.map(
					(owner) =>
						`- ${inlineCode(owner.name)}: removed in ${owner.removed.version} (${formatDate(owner.removed.published)})`,
				)
				.join("\n")}`
}

export function removedMarkdown(api: Api): string {
	return joinBlocks([
		"# Removed",
		removedLead,
		lines("Classes", api.removed?.classes ?? []),
		lines("Enums", api.removed?.enums ?? []),
	])
}

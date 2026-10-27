import type { ItemHistory } from "../../api/item-history.ts"
import type { DisplayTag } from "../../api/tags.ts"
import { formatDate } from "../../api/update-text.ts"

export function datedTags(
	tags: DisplayTag[],
	history: ItemHistory | undefined,
): DisplayTag[] {
	const deprecated = history?.deprecated
	return tags.map((tag) => {
		if (tag.label !== "Deprecated") {
			return tag
		}
		return deprecated === undefined
			? Object.assign({}, tag, {
					hint: "Not recorded: deprecated before the oldest version that this site tracks.",
				})
			: Object.assign({}, tag, {
					note: formatDate(deprecated.published),
					hint: `Deprecated in ${deprecated.version} on ${formatDate(deprecated.published)}.`,
				})
	})
}

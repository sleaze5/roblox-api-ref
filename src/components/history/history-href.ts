import { updateHref } from "../../api/routes.ts"
import type { HistoryEvent } from "../../api/item-history.ts"
import { ownerAnchor } from "../../api/update-text.ts"
import type { OwnerKind } from "../../api/update-types.ts"

export function historyHref(
	event: HistoryEvent,
	kind: OwnerKind,
	owner: string,
): string {
	return `${updateHref(event.version)}#${ownerAnchor({ kind, name: owner })}`
}

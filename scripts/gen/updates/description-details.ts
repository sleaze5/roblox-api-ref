import type { ChangeStatus, DescriptionDetail } from "../../../src/api/update-types.ts"
import type { DescriptionChange } from "../../diff/diff-types.ts"
import { htmlDescriber } from "../api-docs/describer.ts"
import type { ApiNames } from "../api-docs/doc/api-names.ts"

function statusOf(before: unknown, after: unknown): ChangeStatus {
	if (before === undefined) {
		return "Added"
	}
	return after === undefined ? "Removed" : "Changed"
}

export function descriptionDetail(
	names: ApiNames,
	change: DescriptionChange,
): DescriptionDetail | undefined {
	const context =
		change.element === "Class" || change.element === "Member"
			? { kind: "class" as const, name: change.class }
			: { kind: "enum" as const, name: change.enum }
	const describe = htmlDescriber(names, context)
	const convert = (html: string | null) => {
		const doc = html === null ? undefined : describe(html)
		return doc === undefined || doc.length === 0 ? undefined : doc
	}
	const before = convert(change.before)
	const after = convert(change.after)
	if (JSON.stringify(before) === JSON.stringify(after)) {
		return undefined
	}
	const detail: DescriptionDetail = {
		part: change.part,
		status: statusOf(before, after),
	}
	if (change.parameter !== undefined) {
		detail.parameter = change.parameter
	}
	if (before !== undefined) {
		detail.before = before
	}
	if (after !== undefined) {
		detail.after = after
	}
	return detail
}

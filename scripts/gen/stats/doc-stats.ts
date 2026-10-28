import type {
	CoverageKind,
	CoverageRow,
	DocPeriod,
} from "../../../src/api/stats-types.ts"
import type { Api } from "../../../src/api/types.ts"
import type { DescriptionDetail, Update } from "../../../src/api/update-types.ts"

function coverage(api: Api): CoverageRow[] {
	const rows = new Map<CoverageKind, CoverageRow>()
	const add = (kind: CoverageKind, described: boolean) => {
		const row = rows.get(kind) ?? { kind, described: 0, total: 0 }
		row.total++
		row.described += Number(described)
		rows.set(kind, row)
	}
	for (const item of Object.values(api.classes)) {
		add("Class", item.description !== undefined)
		for (const member of item.members) {
			add(member.kind, member.description !== undefined)
			if (member.kind !== "Property") {
				for (const parameter of member.parameters) {
					add("Parameter", parameter.description !== undefined)
				}
			}
			if (
				(member.kind === "Method" || member.kind === "Callback") &&
				member.returns.name !== "void" &&
				member.returns.name !== "null"
			) {
				add("Return", member.returnDescription !== undefined)
			}
		}
	}
	for (const item of Object.values(api.enums)) {
		add("Enum", item.description !== undefined)
		for (const enumItem of item.items) {
			add("EnumItem", enumItem.description !== undefined)
		}
	}
	const order: CoverageKind[] = [
		"Class",
		"Property",
		"Method",
		"Event",
		"Callback",
		"Parameter",
		"Return",
		"Enum",
		"EnumItem",
	]
	return order.flatMap((kind) => {
		const row = rows.get(kind)
		return row === undefined ? [] : [row]
	})
}

function tally(period: DocPeriod, details: DescriptionDetail[]): void {
	for (const detail of details) {
		if (detail.status === "Added") {
			period.added++
		} else if (detail.status === "Removed") {
			period.removed++
		} else {
			period.changed++
		}
	}
}

function years(updates: Update[]): DocPeriod[] {
	const periods = new Map<string, DocPeriod>()
	for (const update of updates.toReversed()) {
		const year = update.published.slice(0, 4)
		const period = periods.get(year) ?? {
			period: year,
			added: 0,
			changed: 0,
			removed: 0,
		}
		for (const owner of update.owners) {
			tally(period, owner.descriptions)
			for (const entry of owner.entries) {
				tally(period, entry.descriptions)
			}
		}
		periods.set(year, period)
	}
	return [...periods.values()]
}

export function docStats(
	api: Api,
	updates: Update[],
): { coverage: CoverageRow[]; years: DocPeriod[] } {
	return { coverage: coverage(api), years: years(updates) }
}

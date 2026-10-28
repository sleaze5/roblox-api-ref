import type { TagTrend, UpdateEvent } from "../../../src/api/stats-types.ts"
import type { Api, Member } from "../../../src/api/types.ts"
import type { ChangeDetail, Update } from "../../../src/api/update-types.ts"
import { compareNames } from "../../../src/api/compare-names.ts"
import { eventOf, oldestFirst } from "./update-refs.ts"

type Values = (item: { tags: string[] } & Partial<Member>) => string[]

class TagTable {
	readonly rows = new Map<string, TagTrend>()

	private row(name: string): TagTrend {
		const row = this.rows.get(name) ?? { name, current: 0, gained: 0, lost: 0 }
		this.rows.set(name, row)
		return row
	}

	count(names: string[]): void {
		for (const name of names) {
			this.row(name).current++
		}
	}

	gain(names: string[], event: UpdateEvent): void {
		for (const name of names) {
			const row = this.row(name)
			row.gained++
			row.first ??= event
		}
	}

	lose(names: string[]): void {
		for (const name of names) {
			this.row(name).lost++
		}
	}

	sorted(): TagTrend[] {
		return [...this.rows.values()].sort(
			(a, b) => b.current - a.current || compareNames(a.name, b.name),
		)
	}
}

function difference(a: string[], b: string[]): string[] {
	return a.filter((name) => !b.includes(name))
}

function detailTags(details: ChangeDetail[], labels: string[]): string[] {
	return details.flatMap((detail) =>
		detail.kind === "tags" && labels.includes(detail.label) ? detail.tags : [],
	)
}

function capabilitiesOf(member: { capabilities?: Member["capabilities"] }): string[] {
	const capabilities = member.capabilities
	return capabilities === undefined
		? []
		: [...new Set([...capabilities.read, ...capabilities.write])]
}

function build(
	api: Api,
	updates: Update[],
	values: Values,
	owners: boolean,
): TagTrend[] {
	const table = new TagTable()
	for (const update of oldestFirst(updates)) {
		const event = eventOf(update)
		for (const owner of update.owners) {
			if (owners) {
				table.gain(detailTags(owner.details, ["Tags", "Added tags"]), event)
				table.lose(detailTags(owner.details, ["Removed tags"]))
			}
			for (const entry of owner.entries) {
				const before = entry.before === undefined ? [] : values(entry.before)
				const after = entry.after === undefined ? [] : values(entry.after)
				table.gain(difference(after, before), event)
				table.lose(difference(before, after))
			}
		}
	}
	for (const item of Object.values(api.classes)) {
		if (owners) {
			table.count(item.tags)
		}
		for (const member of item.members) {
			table.count(values(member))
		}
	}
	for (const item of Object.values(api.enums)) {
		if (owners) {
			table.count(item.tags)
		}
		for (const enumItem of item.items) {
			table.count(values(enumItem))
		}
	}
	return table.sorted()
}

export function tagStats(api: Api, updates: Update[]): TagTrend[] {
	return build(api, updates, (item) => item.tags, true)
}

export function capabilityStats(api: Api, updates: Update[]): TagTrend[] {
	return build(api, updates, capabilitiesOf, false)
}

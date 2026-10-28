import type { RecordGroup, StatRecord, StatRef } from "../../../src/api/stats-types.ts"
import type { Api, MemberKind } from "../../../src/api/types.ts"
import { highest, lowest, type Measured } from "./extremes.ts"

type NameKind = {
	id: string
	plural: string
	refs: (api: Api) => StatRef[]
}

function nameOf(ref: StatRef): string {
	if (ref.kind === "Update") {
		return ref.version
	}
	return ref.kind === "Removed" ? ref.label : ref.name
}

function membersOf(kind: MemberKind): NameKind["refs"] {
	return (api) =>
		Object.values(api.classes).flatMap((item) =>
			item.members
				.filter((member) => member.kind === kind)
				.map((member) => ({ kind, owner: item.name, name: member.name })),
		)
}

const nameKinds: NameKind[] = [
	{
		id: "class",
		plural: "class names",
		refs: (api) =>
			Object.keys(api.classes).map((name) => ({ kind: "Class", name })),
	},
	{ id: "property", plural: "property names", refs: membersOf("Property") },
	{ id: "method", plural: "method names", refs: membersOf("Method") },
	{ id: "event", plural: "event names", refs: membersOf("Event") },
	{ id: "callback", plural: "callback names", refs: membersOf("Callback") },
	{
		id: "parameter",
		plural: "parameter names",
		refs: (api) =>
			Object.values(api.classes).flatMap((item) =>
				item.members.flatMap((member) =>
					member.kind === "Property"
						? []
						: member.parameters.map((parameter) => ({
								kind: "Parameter" as const,
								owner: item.name,
								member: member.name,
								memberKind: member.kind,
								name: parameter.name,
							})),
				),
			),
	},
	{
		id: "enum",
		plural: "enum names",
		refs: (api) => Object.keys(api.enums).map((name) => ({ kind: "Enum", name })),
	},
	{
		id: "enum-item",
		plural: "enum item names",
		refs: (api) =>
			Object.values(api.enums).flatMap((item) =>
				item.items.map((enumItem) => ({
					kind: "EnumItem" as const,
					owner: item.name,
					name: enumItem.name,
				})),
			),
	},
	{
		id: "data-type",
		plural: "data type names",
		refs: (api) =>
			Object.keys(api.dataTypes).map((name) => ({ kind: "DataType", name })),
	},
]

function kindRecords(api: Api, kind: NameKind): StatRecord[] {
	const measured: Measured[] = kind.refs(api).map((ref) => ({
		ref,
		value: nameOf(ref).length,
	}))
	return [
		highest(
			{
				id: `longest-${kind.id}-name`,
				label: `Longest ${kind.plural.replace(/s$/, "")}`,
				hint: `The longest ${kind.plural}, in characters.`,
			},
			measured,
		),
		lowest(
			{
				id: `shortest-${kind.id}-name`,
				label: `Shortest ${kind.plural.replace(/s$/, "")}`,
				hint: `The shortest ${kind.plural}, in characters.`,
			},
			measured,
		),
	]
}

export function nameRecords(api: Api): RecordGroup {
	return {
		id: "name-records",
		title: "Names",
		records: nameKinds.flatMap((kind) => kindRecords(api, kind)),
	}
}

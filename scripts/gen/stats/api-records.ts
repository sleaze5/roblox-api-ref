import { ancestorsOf } from "../../../src/api/members.ts"
import type { RecordGroup } from "../../../src/api/stats-types.ts"
import type { Api, ApiType, Member } from "../../../src/api/types.ts"
import { highest, type Measured } from "./extremes.ts"

function returnCount(type: ApiType): number {
	if (type.name === "void" || type.name === "null") {
		return 0
	}
	return type.name === "Tuple" && type.members !== undefined ? type.members.length : 1
}

function memberRef(owner: string, member: Member): Measured["ref"] {
	return { kind: member.kind, owner, name: member.name }
}

function classRecords(api: Api): RecordGroup {
	const classes = Object.values(api.classes)
	const measure = (value: (item: (typeof classes)[number]) => number): Measured[] =>
		classes.map((item) => ({
			ref: { kind: "Class", name: item.name },
			value: value(item),
		}))
	return {
		id: "class-records",
		title: "Classes",
		records: [
			highest(
				{
					id: "most-superclasses",
					label: "Most superclasses",
					hint: "The deepest classes in the class tree.",
				},
				measure((item) => ancestorsOf(api, item).length),
			),
			highest(
				{
					id: "most-subclasses",
					label: "Most subclasses",
					hint: "Classes that inherit directly from this class.",
				},
				measure((item) => item.subclasses.length),
			),
			highest(
				{
					id: "most-members",
					label: "Most members",
					hint: "Members that the class defines itself.",
				},
				measure((item) => item.members.length),
			),
			highest(
				{
					id: "most-inherited-members",
					label: "Most inherited members",
					hint: "Members that the class inherits from its superclasses.",
				},
				measure((item) =>
					ancestorsOf(api, item).reduce(
						(sum, ancestor) => sum + ancestor.members.length,
						0,
					),
				),
			),
		],
	}
}

function memberRecords(api: Api): RecordGroup {
	const members = Object.values(api.classes).flatMap((item) =>
		item.members.map((member) => ({ owner: item.name, member })),
	)
	return {
		id: "member-records",
		title: "Members",
		records: [
			highest(
				{
					id: "most-parameters",
					label: "Most parameters",
					hint: "The methods, events, and callbacks with the most parameters.",
				},
				members.flatMap(({ owner, member }) =>
					member.kind === "Property"
						? []
						: [
								{
									ref: memberRef(owner, member),
									value: member.parameters.length,
								},
							],
				),
			),
			highest(
				{
					id: "most-return-values",
					label: "Most return values",
					hint: "The methods and callbacks that return the most values.",
				},
				members.flatMap(({ owner, member }) =>
					member.kind === "Method" || member.kind === "Callback"
						? [
								{
									ref: memberRef(owner, member),
									value: returnCount(member.returns),
								},
							]
						: [],
				),
			),
		],
	}
}

function enumAndTypeRecords(api: Api): RecordGroup {
	return {
		id: "type-records",
		title: "Enums and types",
		records: [
			highest(
				{
					id: "most-enum-items",
					label: "Most enum items",
					hint: "The enums with the most items.",
				},
				Object.values(api.enums).map((item) => ({
					ref: { kind: "Enum", name: item.name },
					value: item.items.length,
				})),
			),
			highest(
				{
					id: "most-used-data-type",
					label: "Most-used data type",
					hint: "The data types that the most properties, parameters, and return values use.",
				},
				Object.values(api.dataTypes).map((item) => ({
					ref: { kind: "DataType", name: item.name },
					value: item.usedBy.length,
				})),
			),
			highest(
				{
					id: "most-used-enum",
					label: "Most-used enum",
					hint: "The enums that the most properties, parameters, and return values use.",
				},
				Object.values(api.enums).map((item) => ({
					ref: { kind: "Enum", name: item.name },
					value: item.usedBy.length,
				})),
			),
			highest(
				{
					id: "most-used-class-type",
					label: "Most-used class as a type",
					hint: "The classes that the most properties, parameters, and return values use as their type.",
				},
				Object.values(api.classes).map((item) => ({
					ref: { kind: "Class", name: item.name },
					value: item.usedBy.length,
				})),
			),
		],
	}
}

export function apiRecords(api: Api): RecordGroup[] {
	return [classRecords(api), memberRecords(api), enumAndTypeRecords(api)]
}

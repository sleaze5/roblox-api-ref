import type {
	Dump,
	DumpClass,
	DumpEnum,
	DumpEnumItem,
	DumpMember,
} from "../gen/dump-types.ts"
import { compareFields } from "./compare-fields.ts"
import type {
	ClassChange,
	ClassValue,
	DumpChange,
	EnumChange,
	EnumItemChange,
	EnumValue,
	FieldChange,
	MemberChange,
} from "./diff-types.ts"

type Named = { Name: string }

function byName<T extends Named>(items: T[]): Map<string, T> {
	return new Map(items.map((item) => [item.Name, item]))
}

function pairs<T extends Named>(
	before: T[],
	after: T[],
): { before: T | undefined; after: T | undefined }[] {
	const next = byName(after)
	const previous = byName(before)
	return [
		...before.map((item) => ({ before: item, after: next.get(item.Name) })),
		...after
			.filter((item) => !previous.has(item.Name))
			.map((item) => ({ before: undefined, after: item })),
	]
}

function fieldChanges<T extends object>(
	before: T,
	after: T,
	ignored: readonly string[],
): FieldChange[] {
	return compareFields(
		before as Record<string, unknown>,
		after as Record<string, unknown>,
		ignored,
	)
}

function classValue(item: DumpClass): ClassValue {
	const value: Partial<DumpClass> = { ...item }
	delete value.Members
	return value as ClassValue
}

function enumValue(item: DumpEnum): EnumValue {
	const value: Partial<DumpEnum> = { ...item }
	delete value.Items
	return value as EnumValue
}

function diffMember(
	className: string,
	before: DumpMember | undefined,
	after: DumpMember | undefined,
): MemberChange[] {
	if (before !== undefined && after !== undefined) {
		if (before.MemberType !== after.MemberType) {
			return [
				...diffMember(className, before, undefined),
				...diffMember(className, undefined, after),
			]
		}
		const changes = fieldChanges(before, after, ["Name"])
		return changes.length === 0
			? []
			: [
					{
						element: "Member",
						class: className,
						member: after.Name,
						type: "Change",
						changes,
						value: after,
					},
				]
	}
	if (after !== undefined) {
		return [
			{
				element: "Member",
				class: className,
				member: after.Name,
				type: "Add",
				value: after,
			},
		]
	}
	if (before !== undefined) {
		return [
			{
				element: "Member",
				class: className,
				member: before.Name,
				type: "Remove",
				value: before,
			},
		]
	}
	return []
}

function diffClass(
	before: DumpClass | undefined,
	after: DumpClass | undefined,
): DumpChange[] {
	if (before === undefined && after !== undefined) {
		const added: ClassChange = {
			element: "Class",
			class: after.Name,
			type: "Add",
			value: classValue(after),
		}
		return [
			added,
			...after.Members.flatMap((member) =>
				diffMember(after.Name, undefined, member),
			),
		]
	}
	if (after === undefined && before !== undefined) {
		return [
			{
				element: "Class",
				class: before.Name,
				type: "Remove",
				value: classValue(before),
			},
		]
	}
	if (before === undefined || after === undefined) {
		return []
	}
	const changes = fieldChanges(classValue(before), classValue(after), ["Name"])
	const own: ClassChange[] =
		changes.length === 0
			? []
			: [
					{
						element: "Class",
						class: after.Name,
						type: "Change",
						changes,
						value: classValue(after),
					},
				]
	return [
		...own,
		...pairs(before.Members, after.Members).flatMap((pair) =>
			diffMember(after.Name, pair.before, pair.after),
		),
	]
}

function diffEnumItem(
	enumName: string,
	before: DumpEnumItem | undefined,
	after: DumpEnumItem | undefined,
): EnumItemChange[] {
	if (before !== undefined && after !== undefined) {
		const changes = fieldChanges(before, after, ["Name"])
		return changes.length === 0
			? []
			: [
					{
						element: "EnumItem",
						enum: enumName,
						item: after.Name,
						type: "Change",
						changes,
						value: after,
					},
				]
	}
	if (after !== undefined) {
		return [
			{
				element: "EnumItem",
				enum: enumName,
				item: after.Name,
				type: "Add",
				value: after,
			},
		]
	}
	if (before !== undefined) {
		return [
			{
				element: "EnumItem",
				enum: enumName,
				item: before.Name,
				type: "Remove",
				value: before,
			},
		]
	}
	return []
}

function diffEnum(
	before: DumpEnum | undefined,
	after: DumpEnum | undefined,
): DumpChange[] {
	if (before === undefined && after !== undefined) {
		const added: EnumChange = {
			element: "Enum",
			enum: after.Name,
			type: "Add",
			value: enumValue(after),
		}
		return [
			added,
			...after.Items.flatMap((item) => diffEnumItem(after.Name, undefined, item)),
		]
	}
	if (after === undefined && before !== undefined) {
		return [
			{
				element: "Enum",
				enum: before.Name,
				type: "Remove",
				value: enumValue(before),
			},
		]
	}
	if (before === undefined || after === undefined) {
		return []
	}
	const changes = fieldChanges(enumValue(before), enumValue(after), ["Name"])
	const own: EnumChange[] =
		changes.length === 0
			? []
			: [
					{
						element: "Enum",
						enum: after.Name,
						type: "Change",
						changes,
						value: enumValue(after),
					},
				]
	return [
		...own,
		...pairs(before.Items, after.Items).flatMap((pair) =>
			diffEnumItem(after.Name, pair.before, pair.after),
		),
	]
}

export function diffDump(before: Dump, after: Dump): DumpChange[] {
	return [
		...pairs(before.Classes, after.Classes).flatMap((pair) =>
			diffClass(pair.before, pair.after),
		),
		...pairs(before.Enums, after.Enums).flatMap((pair) =>
			diffEnum(pair.before, pair.after),
		),
	]
}

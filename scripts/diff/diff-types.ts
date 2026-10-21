import type { DescriptionPart } from "../../src/api/update-types.ts"
import type {
	DumpClass,
	DumpEnum,
	DumpEnumItem,
	DumpMember,
} from "../gen/dump-types.ts"

export type ClassValue = Omit<DumpClass, "Members">

export type EnumValue = Omit<DumpEnum, "Items">

export type FieldChange = {
	field: string
	before: unknown
	after: unknown
}

type Change<Value> =
	| { type: "Add"; value: Value }
	| { type: "Remove"; value: Value }
	| { type: "Change"; changes: FieldChange[]; value: Value }

export type ClassChange = { element: "Class"; class: string } & Change<ClassValue>

export type MemberChange = {
	element: "Member"
	class: string
	member: string
} & Change<DumpMember>

export type EnumChange = { element: "Enum"; enum: string } & Change<EnumValue>

export type EnumItemChange = {
	element: "EnumItem"
	enum: string
	item: string
} & Change<DumpEnumItem>

export type DumpChange = ClassChange | MemberChange | EnumChange | EnumItemChange

export type DiffVersion = {
	version: string
	hash: string
}

export type DescriptionTarget =
	| { element: "Class"; class: string }
	| { element: "Member"; class: string; member: string; value: DumpMember }
	| { element: "Enum"; enum: string }
	| { element: "EnumItem"; enum: string; item: string; value: DumpEnumItem }

export type DescriptionChange = DescriptionTarget & {
	part: DescriptionPart
	parameter?: string
	before: string | null
	after: string | null
}

export type DumpDiff = {
	from: DiffVersion
	to: DiffVersion
	changes: DumpChange[]
	descriptions: DescriptionChange[]
}

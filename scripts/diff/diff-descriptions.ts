import type { ApiDocs } from "../gen/api-docs/api-docs-types.ts"
import type { Dump, DumpClass, DumpEnum, DumpMember } from "../gen/dump-types.ts"
import type { DescriptionPart } from "../../src/api/update-types.ts"
import type { DescriptionChange, DescriptionTarget } from "./diff-types.ts"

export type DocumentedDump = {
	dump: Dump
	docs: ApiDocs
}

type Description = {
	target: DescriptionTarget
	part: DescriptionPart
	parameter?: string
	html: string | null
}

type Descriptions = Map<string, Description>

function documentation(docs: ApiDocs, key: string | undefined): string | null {
	return (key === undefined ? undefined : docs[key]?.documentation) || null
}

function targetKey(target: DescriptionTarget): string {
	switch (target.element) {
		case "Class":
			return `Class:${target.class}`
		case "Member":
			return `Member:${target.class}.${target.member}`
		case "Enum":
			return `Enum:${target.enum}`
		case "EnumItem":
			return `EnumItem:${target.enum}.${target.item}`
	}
}

function add(
	descriptions: Descriptions,
	target: DescriptionTarget,
	part: DescriptionPart,
	html: string | null,
	parameter?: string,
): void {
	const description: Description = { target, part, html }
	if (parameter !== undefined) {
		description.parameter = parameter
	}
	descriptions.set(`${targetKey(target)}/${part}/${parameter ?? ""}`, description)
}

function addMember(
	descriptions: Descriptions,
	docs: ApiDocs,
	className: string,
	member: DumpMember,
): void {
	const key = `@roblox/globaltype/${className}.${member.Name}`
	const target: DescriptionTarget = {
		element: "Member",
		class: className,
		member: member.Name,
		value: member,
	}
	add(descriptions, target, "description", documentation(docs, key))
	if (member.MemberType !== "Function" && member.MemberType !== "Callback") {
		return
	}
	const entry = docs[key]
	const parameterKeys = new Map(
		(entry?.params ?? []).map((parameter) => [
			parameter.name,
			parameter.documentation,
		]),
	)
	for (const parameter of member.Parameters) {
		const html = documentation(docs, parameterKeys.get(parameter.Name))
		add(descriptions, target, "parameter", html, parameter.Name)
	}
	add(descriptions, target, "return", documentation(docs, entry?.returns?.[0]))
}

function addClass(descriptions: Descriptions, docs: ApiDocs, item: DumpClass): void {
	const target: DescriptionTarget = { element: "Class", class: item.Name }
	add(
		descriptions,
		target,
		"description",
		documentation(docs, `@roblox/globaltype/${item.Name}`),
	)
	for (const member of item.Members) {
		addMember(descriptions, docs, item.Name, member)
	}
}

function addEnum(descriptions: Descriptions, docs: ApiDocs, item: DumpEnum): void {
	add(
		descriptions,
		{ element: "Enum", enum: item.Name },
		"description",
		documentation(docs, `@roblox/global/Enum.${item.Name}`),
	)
	for (const enumItem of item.Items) {
		add(
			descriptions,
			{
				element: "EnumItem",
				enum: item.Name,
				item: enumItem.Name,
				value: enumItem,
			},
			"description",
			documentation(docs, `@roblox/enum/${item.Name}.${enumItem.Name}`),
		)
	}
}

function collect({ dump, docs }: DocumentedDump): Descriptions {
	const descriptions: Descriptions = new Map()
	for (const item of dump.Classes) {
		addClass(descriptions, docs, item)
	}
	for (const item of dump.Enums) {
		addEnum(descriptions, docs, item)
	}
	return descriptions
}

function change(description: Description, before: string | null): DescriptionChange {
	const fields = {
		part: description.part,
		...(description.parameter === undefined
			? {}
			: { parameter: description.parameter }),
		before,
		after: description.html,
	}
	const target = description.target
	switch (target.element) {
		case "Class":
			return { element: target.element, class: target.class, ...fields }
		case "Enum":
			return { element: target.element, enum: target.enum, ...fields }
		case "Member":
			return {
				element: target.element,
				class: target.class,
				member: target.member,
				...fields,
				value: target.value,
			}
		case "EnumItem":
			return {
				element: target.element,
				enum: target.enum,
				item: target.item,
				...fields,
				value: target.value,
			}
	}
}

export function diffDescriptions(
	before: DocumentedDump,
	after: DocumentedDump,
): DescriptionChange[] {
	const previous = collect(before)
	const changes: DescriptionChange[] = []
	for (const [key, description] of collect(after)) {
		const old = previous.get(key)?.html ?? null
		if (old !== description.html) {
			changes.push(change(description, old))
		}
	}
	return changes
}

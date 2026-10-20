import { inlineTokens, signatureParts } from "../../api/signature.ts"
import { memberTags } from "../../api/tags.ts"
import type { Member, Parameter } from "../../api/types.ts"
import { docMarkdown } from "./doc-markdown.ts"
import { factList, joinBlocks } from "./document.ts"
import { escapeText, inlineCode, link } from "./escape.ts"
import { classMarkdownUrl } from "./markdown-href.ts"

function signature(member: Member, owner: string): string {
	const text = inlineTokens(signatureParts(member, owner))
		.map((part) => part.text)
		.join("")
	return `\`\`\`luau\n${text}\n\`\`\``
}

function parameterList(parameters: Parameter[]): string | undefined {
	const described = parameters.filter(
		(parameter) => parameter.description !== undefined,
	)
	if (described.length === 0) {
		return undefined
	}
	const items = described.map(
		(parameter) =>
			`- ${inlineCode(parameter.name)}: ${docMarkdown(parameter.description)}`,
	)
	return `Parameters:\n\n${items.join("\n")}`
}

function preferred(member: Member): string | undefined {
	const target = member.preferredTarget
	if (member.preferred === undefined || target === undefined) {
		return undefined
	}
	const name = `${target.owner}.${member.preferred}`
	return link(inlineCode(name), classMarkdownUrl(target.owner))
}

function returns(member: Member): string | undefined {
	if (member.kind !== "Method" && member.kind !== "Callback") {
		return undefined
	}
	const text = docMarkdown(member.returnDescription)
	return text === "" ? undefined : `Returns: ${text}`
}

export function memberMarkdown(member: Member, owner: string): string {
	const tags = memberTags(member).map((tag) => tag.label)
	return joinBlocks([
		`### ${escapeText(member.name)}`,
		signature(member, owner),
		factList([
			["Tags", tags.join(", ")],
			["Use instead", preferred(member)],
		]),
		docMarkdown(member.description),
		member.kind === "Property" ? undefined : parameterList(member.parameters),
		returns(member),
	]).trimEnd()
}

import type { DocNode } from "../../../../src/api/doc-types.ts"

export function normalizeText(text: string): string {
	return text
		.replaceAll(/\r\n?/g, "\n")
		.replaceAll(/[\u2018\u2019\u201A\u201B]/g, "'")
		.replaceAll(/[\u201C\u201D\u201E\u201F]/g, '"')
		.replaceAll(/\n{3,}/g, "\n\n")
}

export function normalizeNodes(nodes: DocNode[]): DocNode[] {
	return nodes.map((node) =>
		typeof node === "string"
			? normalizeText(node)
			: { ...node, children: normalizeNodes(node.children) },
	)
}

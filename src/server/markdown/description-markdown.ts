import type { DescriptionDetail } from "../../api/update-types.ts"
import { docMarkdown } from "./doc-markdown.ts"
import { inlineCode } from "./escape.ts"

function label(detail: DescriptionDetail): string {
	switch (detail.part) {
		case "description":
			return "Description"
		case "return":
			return "Return value description"
		case "parameter":
			return `Description of ${inlineCode(detail.parameter ?? "")}`
	}
}

export function descriptionLines(
	details: DescriptionDetail[],
	indent: string,
): string[] {
	return details.map((detail) => {
		const text = `${label(detail)} ${detail.status.toLowerCase()}`
		const after = docMarkdown(detail.after)
			.replaceAll(/\s*\n+\s*/g, " ")
			.trim()
		return `${indent}- ${after === "" ? `${text}.` : `${text}: ${after}`}`
	})
}

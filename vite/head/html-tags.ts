import { escapeHtml } from "../escape-html.ts"

export function meta(
	attribute: "name" | "property",
	key: string,
	content: string,
): string {
	return `<meta ${attribute}="${key}" content="${escapeHtml(content)}">`
}

export function link(attributes: Record<string, string>): string {
	const pairs = Object.entries(attributes).map(
		([key, value]) => `${key}="${escapeHtml(value)}"`,
	)
	return `<link ${pairs.join(" ")}>`
}

export function escapeText(text: string): string {
	return text.replaceAll(/[`*_[\]<]/g, "\\$&")
}

export function inlineCode(text: string): string {
	const fence = text.includes("`") ? "``" : "`"
	return `${fence}${text}${fence}`
}

export function link(text: string, url: string): string {
	return `[${text}](${url.replaceAll(" ", "%20")})`
}

export function tableCell(markdown: string): string {
	return markdown.replaceAll(/\s*\n+\s*/g, " ").replaceAll("|", "\\|")
}

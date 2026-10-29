const entities: Record<string, string> = {
	"&": "&amp;",
	"<": "&lt;",
	">": "&gt;",
	'"': "&quot;",
}

export function escapeHtml(text: string): string {
	return text.replaceAll(/[&<>"]/g, (character) => entities[character] ?? character)
}

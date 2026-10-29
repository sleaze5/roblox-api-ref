const entities: Record<string, string> = {
	"&": "&amp;",
	"<": "&lt;",
	">": "&gt;",
	'"': "&quot;",
}

/** Escapes text for HTML content and double-quoted attribute values. */
export function escapeHtml(text: string): string {
	return text.replaceAll(/[&<>"]/g, (character) => entities[character] ?? character)
}

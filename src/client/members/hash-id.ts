export function hashId(hash: string): string {
	try {
		return decodeURIComponent(hash.slice(1))
	} catch {
		return ""
	}
}

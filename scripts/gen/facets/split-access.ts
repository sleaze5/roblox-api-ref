import type { FacetAccess } from "../../../src/api/types.ts"

export type AccessValue = {
	name: string
	access?: FacetAccess
}

export function splitAccess(read: string[], write: string[]): AccessValue[] {
	const writeSet = new Set(write)
	const readSet = new Set(read)
	return [...new Set([...read, ...write])].map((name) =>
		readSet.has(name) && writeSet.has(name)
			? { name }
			: { name, access: readSet.has(name) ? "Read" : "Write" },
	)
}

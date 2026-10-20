import { scoreName } from "../../search/score.ts"

export function matchesName(query: string, name: string): boolean {
	return query === "" || scoreName(query, name, name.toLowerCase()) > 0
}

import type { EnumItem } from "../../src/api/types.ts"
import type { DumpEnumItem } from "./dump-types.ts"
import { normalizeTags } from "./normalize-member.ts"

export function normalizeEnumItem(item: DumpEnumItem): EnumItem {
	return {
		name: item.Name,
		value: item.Value,
		tags: normalizeTags(item.Tags),
		legacyNames: item.LegacyNames ?? [],
	}
}

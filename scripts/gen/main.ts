import { join } from "node:path"
import { iconNames, maskIconNames } from "../../src/components/icon-names.ts"
import { dataDir, generatedDir } from "../paths.ts"
import { apiDocsFileName, dumpFileName, readVersion } from "../roblox-version.ts"
import type { ApiDocs } from "./api-docs/api-docs-types.ts"
import type { Dump } from "./dump-types.ts"
import { buildFavicon } from "./favicon.ts"
import { buildFacetSlugs } from "./facets/facet-slugs.ts"
import { buildIconSprite, readIcon } from "./icon-sprite.ts"
import { normalizeApi } from "./normalize-api.ts"
import { buildSearchIndex } from "./search-index.ts"
import { subsetFonts } from "./subset-fonts.ts"

const version = await readVersion()
if (version === undefined) {
	throw new Error("roblox-version.json is missing. Run `vp run dump:update` first.")
}

const dump = (await Bun.file(join(dataDir, dumpFileName(version))).json()) as Dump
const docs = (await Bun.file(join(dataDir, apiDocsFileName(version))).json()) as ApiDocs
const api = normalizeApi(dump, docs, version)
const searchIndex = buildSearchIndex(api)

await Promise.all([
	Bun.write(join(generatedDir, "api.json"), JSON.stringify(api)),
	Bun.write(
		join(generatedDir, "facet-slugs.json"),
		JSON.stringify(buildFacetSlugs(api.facets)),
	),
	Bun.write(join(generatedDir, "search.json"), JSON.stringify(searchIndex)),
	Bun.write(join(generatedDir, "icons.svg"), await buildIconSprite(iconNames)),
	subsetFonts(),
	Bun.write(join(generatedDir, "favicon.svg"), await buildFavicon()),
	...maskIconNames.map(async (name) =>
		Bun.write(join(generatedDir, "icons", `${name}.svg`), await readIcon(name)),
	),
])

console.log(
	`Generated ${Object.keys(api.classes).length} classes, ${Object.keys(api.enums).length} enums, ${Object.keys(api.dataTypes).length} data types, and ${searchIndex.names.length} search entries for ${version.channel} ${version.version}.`,
)

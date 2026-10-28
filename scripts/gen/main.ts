import { join } from "node:path"
import { iconNames, maskIconNames } from "../../src/components/icon-names.ts"
import { dataDir, generatedDir } from "../paths.ts"
import {
	apiDocsFileName,
	dumpFileName,
	readVersions,
	studioIconsFileName,
} from "../roblox-version.ts"
import type { ApiDocs } from "./api-docs/api-docs-types.ts"
import { applyClassIcons } from "./class-icons/class-icons.ts"
import type { StudioIcons } from "./class-icons/studio-icons-types.ts"
import type { Dump } from "./dump-types.ts"
import { buildFavicon } from "./favicon.ts"
import { buildFacetSlugs } from "./facets/facet-slugs.ts"
import { buildIconSprite, readIcon } from "./icon-sprite.ts"
import { normalizeApi } from "./normalize-api.ts"
import { buildSearchIndex } from "./search-index.ts"
import { subsetFonts } from "./subset-fonts.ts"
import { applyHistory } from "./history/apply-history.ts"
import { buildHistory } from "./history/history-table.ts"
import { buildUpdates, readDiffs } from "./updates/read-updates.ts"

const versions = await readVersions()
const version = versions.at(-1)
if (version === undefined) {
	throw new Error(
		"roblox-versions.json has no versions. Run `bun vp run dump:update` first.",
	)
}

const dump = (await Bun.file(join(dataDir, dumpFileName(version))).json()) as Dump
const docs = (await Bun.file(join(dataDir, apiDocsFileName(version))).json()) as ApiDocs
const studioIcons = (await Bun.file(
	join(dataDir, studioIconsFileName(version)),
).json()) as StudioIcons
const api = normalizeApi(dump, docs, version)
const classIcons = applyClassIcons(api, studioIcons)
const diffs = await readDiffs(versions)
const updates = buildUpdates(diffs, api)
applyHistory(api, buildHistory(updates), versions[0]?.version ?? version.version)
const searchIndex = buildSearchIndex(api, updates)

await Promise.all([
	Bun.write(join(generatedDir, "api.json"), JSON.stringify(api)),
	Bun.write(join(generatedDir, "updates.json"), JSON.stringify(updates)),
	Bun.write(
		join(generatedDir, "facet-slugs.json"),
		JSON.stringify(buildFacetSlugs(api.facets)),
	),
	Bun.write(join(generatedDir, "search.json"), JSON.stringify(searchIndex)),
	Bun.write(
		join(generatedDir, "build.json"),
		JSON.stringify({ lastModified: new Date().toUTCString() }),
	),
	Bun.write(join(generatedDir, "icons.svg"), await buildIconSprite(iconNames)),
	Bun.write(join(generatedDir, "class-icons.png"), classIcons.png),
	Bun.write(join(generatedDir, "class-icons.css"), classIcons.css),
	subsetFonts(),
	Bun.write(join(generatedDir, "favicon.svg"), await buildFavicon()),
	...maskIconNames.map(async (name) =>
		Bun.write(join(generatedDir, "icons", `${name}.svg`), await readIcon(name)),
	),
])

console.log(
	`Generated ${Object.keys(api.classes).length} classes, ${Object.keys(api.enums).length} enums, ${Object.keys(api.dataTypes).length} data types, ${searchIndex.names.length} search entries, and ${updates.length} updates for ${version.channel} ${version.version}.`,
)

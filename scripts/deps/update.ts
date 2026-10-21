import { join } from "node:path"
import { rootDir } from "../paths.ts"
import { fetchVersions } from "./npm-versions.ts"
import { parseSpec } from "./package-spec.ts"
import { parseVersion, safeUpgrade } from "./semver.ts"

type PackageJson = {
	dependencies?: Record<string, string>
	devDependencies?: Record<string, string>
	overrides?: Record<string, string>
	devEngines?: { packageManager?: { name: string; version: string } }
}

type Change = {
	label: string
	from: string
	to: string
}

const packagePath = join(rootDir, "package.json")
const manifest = (await Bun.file(packagePath).json()) as PackageJson
const versionCache = new Map<string, Promise<string[]>>()
const changes: Change[] = []
const held: string[] = []

function versionsOf(name: string): Promise<string[]> {
	let versions = versionCache.get(name)
	if (versions === undefined) {
		versions = fetchVersions(name)
		versionCache.set(name, versions)
	}
	return versions
}

async function upgrade(
	label: string,
	name: string,
	current: string,
): Promise<Change | undefined> {
	if (parseVersion(current)?.prerelease !== undefined) {
		held.push(`${label} ${current}`)
		return undefined
	}
	const next = safeUpgrade(current, await versionsOf(name))
	return next === undefined ? undefined : { label, from: current, to: next }
}

function record(change: Change | undefined, apply: (version: string) => void): void {
	if (change !== undefined) {
		changes.push(change)
		apply(change.to)
	}
}

async function upgradeSection(
	section: Record<string, string> | undefined,
): Promise<void> {
	if (section === undefined) {
		return
	}
	const specs = Object.entries(section).map(([key, spec]) => ({
		key,
		spec: parseSpec(key, spec),
	}))
	const results = await Promise.all(
		specs.map(({ key, spec }) => upgrade(key, spec.name, spec.version)),
	)
	specs.forEach(({ key, spec }, index) => {
		record(results[index], (version) => {
			section[key] = spec.format(version)
		})
	})
}

async function upgradePackageManager(): Promise<void> {
	const packageManager = manifest.devEngines?.packageManager
	if (packageManager === undefined) {
		return
	}
	const change = await upgrade(
		packageManager.name,
		packageManager.name,
		packageManager.version,
	)
	record(change, (version) => {
		packageManager.version = version
	})
}

await upgradeSection(manifest.dependencies)
await upgradeSection(manifest.devDependencies)
await upgradeSection(manifest.overrides)
await upgradePackageManager()

if (changes.length > 0) {
	await Bun.write(packagePath, `${JSON.stringify(manifest, null, "\t")}\n`)
}

const lines = [
	...changes.map((change) => `- \`${change.label}\` ${change.from} → ${change.to}`),
	...(held.length === 0
		? []
		: [
				"",
				"Prerelease versions are left as they are:",
				...held.map((entry) => `- \`${entry}\``),
			]),
]
console.log(
	changes.length === 0
		? "Every dependency is up to date."
		: `Updated ${changes.length} ${changes.length === 1 ? "dependency" : "dependencies"}.`,
)
console.log(lines.join("\n"))
const summaryFile = process.env["DEPS_SUMMARY_FILE"]
if (summaryFile !== undefined && summaryFile !== "") {
	await Bun.write(summaryFile, `${lines.join("\n")}\n`)
}

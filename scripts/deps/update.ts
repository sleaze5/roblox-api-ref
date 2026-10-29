import { join } from "node:path"
import { parseArgs } from "node:util"
import { rootDir } from "../paths.ts"
import { skippedPackages } from "./dependabot.ts"
import { fetchVersions } from "./npm-versions.ts"
import {
	fetchStableRust,
	setRustVersion,
	setToolchainChannel,
	toolchainChannel,
} from "./rust-toolchain.ts"
import { parseSpec } from "./package-spec.ts"
import { safeUpgrade } from "./semver.ts"

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

const { values } = parseArgs({
	options: {
		all: { type: "boolean", default: false },
		uncovered: { type: "boolean", default: false },
	},
})
if (values.all && values.uncovered) {
	throw new Error("Pass either --all or --uncovered, not both.")
}
const skipped = values.uncovered
	? skippedPackages(await Bun.file(join(rootDir, ".github/dependabot.yml")).text())
	: undefined

const packagePath = join(rootDir, "package.json")
const manifest = (await Bun.file(packagePath).json()) as PackageJson
const versionCache = new Map<string, Promise<string[]>>()
const changes: Change[] = []

function versionsOf(name: string): Promise<string[]> {
	let versions = versionCache.get(name)
	if (versions === undefined) {
		versions =
			name === "rust"
				? fetchStableRust().then((version) => [version])
				: fetchVersions(name)
		versionCache.set(name, versions)
	}
	return versions
}

async function upgrade(
	label: string,
	name: string,
	current: string,
): Promise<Change | undefined> {
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
	const specs = Object.entries(section)
		.filter(([key]) => skipped?.has(key) ?? true)
		.map(([key, spec]) => ({ key, spec: parseSpec(key, spec) }))
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

async function upgradeRust(): Promise<void> {
	const toolchainPath = join(rootDir, "rust-toolchain.toml")
	const cargoPath = join(rootDir, "Cargo.toml")
	const toolchain = await Bun.file(toolchainPath).text()
	const current = toolchainChannel(toolchain)
	if (current === undefined) {
		throw new Error("rust-toolchain.toml does not pin a channel.")
	}
	const change = await upgrade("rust", "rust", current)
	if (change === undefined) {
		return
	}
	changes.push(change)
	await Bun.write(toolchainPath, setToolchainChannel(toolchain, change.to))
	await Bun.write(
		cargoPath,
		setRustVersion(await Bun.file(cargoPath).text(), change.to),
	)
}

await upgradeSection(manifest.dependencies)
await upgradeSection(manifest.devDependencies)
await upgradeSection(manifest.overrides)
await upgradePackageManager()
await upgradeRust()

if (changes.some((change) => change.label !== "rust")) {
	await Bun.write(packagePath, `${JSON.stringify(manifest, null, "\t")}\n`)
}

const lines = changes.map(
	(change) => `- \`${change.label}\` ${change.from} → ${change.to}`,
)
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

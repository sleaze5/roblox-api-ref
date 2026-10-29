import { readdirSync, readFileSync, statSync } from "node:fs"
import { join } from "node:path"
import { describe, expect, test } from "bun:test"

const root = join(import.meta.dir, "..", "..")
const dist = join(root, "dist")

const markers = new Set(["group/rail"])

function files(dir: string, extension: string): string[] {
	return readdirSync(dir).flatMap((name) => {
		const path = join(dir, name)
		if (statSync(path).isDirectory()) {
			return files(path, extension)
		}
		return path.endsWith(extension) ? [path] : []
	})
}

function cssClasses(): Set<string> {
	const css = files(join(dist, "assets"), ".css")
		.map((path) => readFileSync(path, "utf8"))
		.join("\n")
	const classes = new Set<string>()
	for (const match of css.matchAll(/\.((?:\\.|[\w-])+)/g)) {
		classes.add((match[1] ?? "").replaceAll(/\\(.)/g, "$1"))
	}
	return classes
}

function htmlClasses(html: string): string[] {
	return [...html.matchAll(/\sclass=(?:"([^"]*)"|([^\s>"]+))/g)].flatMap((match) =>
		(match[1] ?? match[2] ?? "").split(/\s+/),
	)
}

function sourceClasses(): string[] {
	return files(join(root, "src", "client"), ".ts")
		.filter((path) => !path.endsWith(".test.ts"))
		.flatMap((path) => {
			const source = readFileSync(path, "utf8")
			const literals = [
				...source.matchAll(/class="([^"$]*)"/g),
				...source.matchAll(/(?:Class|class(?:Name)?)\s*=\s*\n?\s*"([^"]*)"/g),
			]
			return literals.flatMap((match) => (match[1] ?? "").split(/\s+/))
		})
}

describe("class names", () => {
	const known = cssClasses()
	const unknown = (classes: Iterable<string>) =>
		[...new Set(classes)]
			.filter((name) => name !== "" && !name.includes("${"))
			.filter((name) => !known.has(name) && !markers.has(name))
			.sort()

	test("every class on every built page has CSS rules", () => {
		const used = new Set<string>()
		for (const path of files(dist, ".html")) {
			for (const name of htmlClasses(readFileSync(path, "utf8"))) {
				used.add(name)
			}
		}
		expect(used.size).toBeGreaterThan(200)
		expect(unknown(used)).toEqual([])
	})

	test("every class that the client scripts add has CSS rules", () => {
		const used = sourceClasses()
		expect(used.length).toBeGreaterThan(50)
		expect(unknown(used)).toEqual([])
	})
})

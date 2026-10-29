import { readdirSync, readFileSync, statSync } from "node:fs"
import { join } from "node:path"
import { brotliCompressSync, constants, gzipSync } from "node:zlib"
import init, { Matcher, Search } from "../src/generated/search.js"
import { generatedDir, rootDir } from "./paths.ts"

const queries = [
	"Part",
	"part",
	"gtp",
	"Humanoid.Move",
	"Part.",
	".Touched",
	"is:enum Material",
	"tag:yields !tag:deprecated",
	"type:Color3 is:property",
	"uses:Enum.KeyCode",
	"access:game Get",
	'"Name"',
	"Enum.Material.Plas",
	"version-7062",
	"0.74",
	"workspace",
	"CFrame",
	"GetChildren",
	"zzqqxx",
	"x",
]

type Run = { ms: number; peakMb: number; output: string }

function run(command: string[], env: Record<string, string> = {}): Run {
	const start = performance.now()
	const result = Bun.spawnSync(command, {
		cwd: rootDir,
		env: { ...process.env, ...env },
		stdout: "pipe",
		stderr: "pipe",
	})
	const ms = performance.now() - start
	if (!result.success) {
		throw new Error(`${command.join(" ")} failed:\n${result.stderr.toString()}`)
	}
	const peakBytes = result.resourceUsage?.maxRSS ?? 0
	return {
		ms,
		peakMb: peakBytes / 1024 / 1024,
		output: result.stdout.toString().trim(),
	}
}

function sizes(bytes: Uint8Array): string {
	const brotli = brotliCompressSync(bytes, {
		params: { [constants.BROTLI_PARAM_QUALITY]: 11 },
	}).length
	return `${kb(bytes.length)} raw, ${kb(gzipSync(bytes, { level: 9 }).length)} gzip, ${kb(brotli)} brotli`
}

function kb(bytes: number): string {
	return `${(bytes / 1024).toFixed(1)} KB`
}

function treeSize(dir: string, extension: string): number {
	return readdirSync(dir).reduce((total, name) => {
		const path = join(dir, name)
		const stats = statSync(path)
		if (stats.isDirectory()) {
			return total + treeSize(path, extension)
		}
		return path.endsWith(extension) ? total + stats.size : total
	}, 0)
}

function timePerQuery(search: (query: string) => unknown, passes: number): number {
	for (const query of queries) {
		search(query)
	}
	const start = performance.now()
	for (let pass = 0; pass < passes; pass++) {
		for (const query of queries) {
			search(query)
		}
	}
	return (performance.now() - start) / (passes * queries.length)
}

console.log("## Build\n")
run(["cargo", "build", "--release", "--quiet", "--package", "rapi"])
const gen = run(["cargo", "rapi", "gen"])
console.log(
	`- Generation (cargo rapi gen): ${gen.ms.toFixed(0)} ms, peak ${gen.peakMb.toFixed(0)} MB`,
)
const benchDist = join(rootDir, "node_modules", ".cache", "bench-dist")
const client = run(["bun", "vp", "build", "--outDir", benchDist, "--emptyOutDir"], {
	RAPI_RENDER: "0",
})
console.log(`- Client build (Vite): ${client.ms.toFixed(0)} ms`)
const render = run(["cargo", "rapi", "render", "--dist", benchDist])
console.log(
	`- Prerender (cargo rapi render, with generation): ${render.ms.toFixed(0)} ms, peak ${render.peakMb.toFixed(0)} MB`,
)
const dist = benchDist

console.log("\n## Search\n")
await init({ module_or_path: readFileSync(join(generatedDir, "search_bg.wasm")) })
const indexBytes = new Uint8Array(readFileSync(join(generatedDir, "search.bin")))
const loadStart = performance.now()
const search = new Search(indexBytes)
console.log(`- Index load: ${(performance.now() - loadStart).toFixed(1)} ms`)
console.log(
	`- Query (WebAssembly, ${queries.length} queries): ${timePerQuery((query) => search.view(query), 50).toFixed(3)} ms per query`,
)
const pages = JSON.parse(readFileSync(join(generatedDir, "pages.json"), "utf8")) as [
	string,
	string,
][]
const names = pages
	.map(([path]) => decodeURIComponent(path.split("/").at(-2) ?? ""))
	.slice(0, 2000)
const matcher = new Matcher(names.join("\n"))
console.log(
	`- Filter match of ${names.length} names: ${timePerQuery((query) => matcher.matches(query), 50).toFixed(3)} ms per query`,
)

console.log("\n## Sizes\n")
console.log(`- search.bin: ${sizes(indexBytes)}`)
console.log(
	`- search_bg.wasm: ${sizes(new Uint8Array(readFileSync(join(generatedDir, "search_bg.wasm"))))}`,
)
{
	console.log(
		`- dist HTML: ${(treeSize(dist, ".html") / 1024 / 1024).toFixed(1)} MB, Markdown: ${(treeSize(dist, ".md") / 1024 / 1024).toFixed(1)} MB`,
	)
	const assets = join(dist, "assets")
	for (const name of readdirSync(assets).filter((file) => /\.(js|css)$/.test(file))) {
		console.log(
			`- ${name.replace(/-[\w-]{8}\./, ".")}: ${sizes(new Uint8Array(readFileSync(join(assets, name))))}`,
		)
	}
}

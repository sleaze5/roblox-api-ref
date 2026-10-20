import { mkdir, readFile, rm, writeFile } from "node:fs/promises"
import { dirname, join } from "node:path"
import { pathToFileURL } from "node:url"
import type { PageModule } from "../src/server/page-module.ts"
import { compressibleFiles } from "./compress.ts"
import { minifyTemplateScripts } from "./minify-html.ts"
import { OutputPool } from "./output-pool.ts"
import type { Site } from "./site.ts"
import { openSearchXml, robotsTxt, sitemapXml } from "./site-files.ts"
import { fillTemplate } from "./template.ts"

export type PrerenderPaths = {
	outDir: string
	serverEntry: string
	workerFile: string
}

function outputFile(outDir: string, route: string): string {
	if (route === "/404") {
		return join(outDir, "404.html")
	}
	return join(outDir, decodeURIComponent(route), "index.html")
}

function markdownFile(outDir: string, site: Site, href: string): string {
	return join(outDir, decodeURIComponent(href.slice(site.base.length)))
}

async function writeSiteFiles(
	outDir: string,
	site: Site,
	routes: string[],
	module: PageModule,
): Promise<void> {
	const files: [string, string | undefined][] = [
		["robots.txt", robotsTxt(site, routes)],
		["sitemap.xml", sitemapXml(site, routes, module.lastModified())],
		["opensearch.xml", openSearchXml(site)],
		["llms.txt", module.llmsTxt()],
	]
	await Promise.all(
		files.map(async ([name, content]) => {
			if (content !== undefined) {
				await writeFile(join(outDir, name), content)
			}
		}),
	)
}

export async function prerender(paths: PrerenderPaths, site: Site): Promise<void> {
	const start = performance.now()
	const module = (await import(pathToFileURL(paths.serverEntry).href)) as PageModule
	const template = await minifyTemplateScripts(
		await readFile(join(paths.outDir, "index.html"), "utf8"),
	)
	const routes = module.routes()
	await rm(join(paths.outDir, "index.html"))
	await writeSiteFiles(paths.outDir, site, routes, module)
	const assets = await compressibleFiles(paths.outDir)
	const pool = new OutputPool(paths.workerFile)
	try {
		await Promise.all([
			...routes.map(async (route) => {
				const file = outputFile(paths.outDir, route)
				await mkdir(dirname(file), { recursive: true })
				const page = module.renderPage(route)
				if (page.markdown !== undefined) {
					await writeFile(
						markdownFile(paths.outDir, site, page.markdown.href),
						page.markdown.content,
					)
				}
				await pool.run({
					file,
					html: fillTemplate(template, page, route, site),
				})
			}),
			...assets.map((file) => pool.run({ file })),
		])
	} finally {
		await pool.close()
	}
	await rm(dirname(paths.serverEntry), { recursive: true, force: true })
	console.log(
		`Prerendered, minified, and compressed ${routes.length} pages and ${assets.length} assets in ${Math.round(performance.now() - start)}ms.`,
	)
}

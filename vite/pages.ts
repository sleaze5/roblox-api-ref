import { readFile } from "node:fs/promises"
import { resolve } from "node:path"
import { isRunnableDevEnvironment, type Plugin, type ViteDevServer } from "vite-plus"
import type { PageModule } from "../src/server/page-module.ts"
import { prerender } from "./prerender.ts"
import { servePages } from "./preview.ts"
import { fillTemplate } from "./template.ts"

const serverEntry = "src/server/render.tsx"
const serverOutDir = "node_modules/.cache/pages-server"

async function loadDevModule(server: ViteDevServer): Promise<PageModule> {
	const environment = server.environments["ssr"]
	if (environment === undefined || !isRunnableDevEnvironment(environment)) {
		throw new Error("The ssr environment is not runnable.")
	}
	return environment.runner.import<PageModule>(`/${serverEntry}`)
}

function isPageRequest(url: string, accept: string | undefined): boolean {
	const path = url.split("?")[0] ?? ""
	return (accept?.includes("text/html") ?? false) && !/\.[a-z0-9]+$/i.test(path)
}

export function pages(): Plugin {
	let root = process.cwd()
	return {
		name: "pages",
		config() {
			return {
				appType: "custom",
				environments: {
					ssr: {
						build: {
							outDir: serverOutDir,
							emptyOutDir: true,
							copyPublicDir: false,
							rollupOptions: { input: serverEntry },
						},
					},
				},
				builder: {
					async buildApp(builder) {
						const client = builder.environments["client"]
						const ssr = builder.environments["ssr"]
						if (client === undefined || ssr === undefined) {
							throw new Error(
								"The client and ssr environments are required.",
							)
						}
						await builder.build(client)
						await builder.build(ssr)
						await prerender({
							outDir: resolve(root, client.config.build.outDir),
							serverEntry: resolve(root, serverOutDir, "render.js"),
							workerFile: resolve(root, "vite", "output-worker.ts"),
						})
					},
				},
			}
		},
		configResolved(config) {
			root = config.root
		},
		configurePreviewServer(server) {
			servePages(server, resolve(root, server.config.build.outDir))
		},
		configureServer(server) {
			return () => {
				server.middlewares.use((request, response, next) => {
					const url = request.originalUrl ?? request.url ?? "/"
					if (
						request.method !== "GET" ||
						!isPageRequest(url, request.headers.accept)
					) {
						next()
						return
					}
					renderDevPage(server, url).then((page) => {
						response.statusCode = page.status
						response.setHeader("Content-Type", "text/html; charset=utf-8")
						response.end(page.html)
						return undefined
					}, next)
				})
			}
		},
	}

	async function renderDevPage(
		server: ViteDevServer,
		url: string,
	): Promise<{ status: number; html: string }> {
		const module = await loadDevModule(server)
		const path = url.split("?")[0] ?? "/"
		const page = module.renderPage(path)
		const template = await server.transformIndexHtml(
			url,
			await readFile(resolve(root, "index.html"), "utf8"),
		)
		return { status: page.status, html: fillTemplate(template, page, path) }
	}
}

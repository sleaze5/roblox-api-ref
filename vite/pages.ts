import { readFile, rm } from "node:fs/promises"
import { join, resolve } from "node:path"
import type { Plugin } from "vite-plus"
import { servePages } from "./preview.ts"
import { DevRenderer, runRapi } from "./rapi.ts"

function isPageRequest(url: string, accept: string | undefined): boolean {
	const path = url.split("?")[0] ?? ""
	return (accept?.includes("text/html") ?? false) && !/\.[a-z0-9]+$/i.test(path)
}

/** Files whose change means the dev server must reload the site data or renderer. */
const rendererInputs =
	/(^|[\\/])(crates[\\/].*\.rs|roblox-versions\.json|data[\\/]|diffs[\\/])/

/**
 * Prerenders every page with the Rust renderer after the client build, and
 * renders pages on request on the dev server.
 */
export function pages(): Plugin {
	let root = process.cwd()
	return {
		name: "pages",
		config() {
			return {
				appType: "custom",
				build: { manifest: true },
				builder: {
					async buildApp(builder) {
						const client = builder.environments["client"]
						if (client === undefined) {
							throw new Error("The client environment is required.")
						}
						await builder.build(client)
						const outDir = resolve(root, client.config.build.outDir)
						await runRapi(root, ["render", "--dist", outDir])
						await rm(join(outDir, ".vite"), {
							recursive: true,
							force: true,
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
			const renderer = new DevRenderer(root)
			server.watcher.add(["crates", "data", "diffs", "roblox-versions.json"])
			server.watcher.on("change", (file) => {
				if (rendererInputs.test(file.slice(root.length))) {
					renderer.restart()
					server.ws.send({ type: "full-reload" })
				}
			})
			server.httpServer?.on("close", () => renderer.restart())
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
					const path = url.split("?")[0] ?? "/"
					readFile(resolve(root, "index.html"), "utf8")
						.then((html) => server.transformIndexHtml(url, html))
						.then((template) => renderer.render(path, template))
						.then((page) => {
							response.statusCode = page.status
							response.setHeader(
								"Content-Type",
								"text/html; charset=utf-8",
							)
							response.end(page.html)
							return undefined
						}, next)
				})
			}
		},
	}
}

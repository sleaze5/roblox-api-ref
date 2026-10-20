import { readFile, stat } from "node:fs/promises"
import type { IncomingMessage, ServerResponse } from "node:http"
import { extname, join, normalize } from "node:path"
import type { PreviewServer } from "vite-plus"

const contentTypes: Record<string, string> = {
	".html": "text/html; charset=utf-8",
	".js": "text/javascript; charset=utf-8",
	".css": "text/css; charset=utf-8",
	".json": "application/json; charset=utf-8",
	".svg": "image/svg+xml",
	".xml": "application/xml; charset=utf-8",
	".txt": "text/plain; charset=utf-8",
	".md": "text/markdown; charset=utf-8",
	".png": "image/png",
	".woff2": "font/woff2",
}

const encodings = [
	{ name: "br", extension: ".br" },
	{ name: "zstd", extension: ".zst" },
	{ name: "gzip", extension: ".gz" },
]

type StaticFile = {
	status: number
	path: string
}

async function isFile(path: string): Promise<boolean> {
	try {
		return (await stat(path)).isFile()
	} catch {
		return false
	}
}

async function resolveFile(
	outDir: string,
	urlPath: string,
): Promise<StaticFile | undefined> {
	const target = normalize(
		join(outDir, urlPath.endsWith("/") ? `${urlPath}index.html` : urlPath),
	)
	if (target.startsWith(outDir) && (await isFile(target))) {
		return { status: 200, path: target }
	}
	if (urlPath.endsWith("/") || extname(urlPath) === "") {
		return { status: 404, path: join(outDir, "404.html") }
	}
	return undefined
}

async function encodedVariant(
	path: string,
	accepted: string,
): Promise<{ path: string; encoding?: string }> {
	const candidates = encodings.filter((encoding) => accepted.includes(encoding.name))
	const available = await Promise.all(
		candidates.map((encoding) => isFile(`${path}${encoding.extension}`)),
	)
	const match = candidates.find((_, index) => available[index])
	return match === undefined
		? { path }
		: { path: `${path}${match.extension}`, encoding: match.name }
}

async function respond(
	outDir: string,
	request: IncomingMessage,
	response: ServerResponse,
): Promise<boolean> {
	const urlPath = decodeURIComponent((request.url ?? "/").split("?")[0] ?? "/")
	const file = await resolveFile(outDir, urlPath)
	if (file === undefined) {
		return false
	}
	const variant = await encodedVariant(
		file.path,
		request.headers["accept-encoding"] ?? "",
	)
	response.statusCode = file.status
	response.setHeader(
		"Content-Type",
		contentTypes[extname(file.path)] ?? "application/octet-stream",
	)
	response.setHeader("Vary", "Accept-Encoding")
	if (variant.encoding !== undefined) {
		response.setHeader("Content-Encoding", variant.encoding)
	}
	response.end(request.method === "HEAD" ? undefined : await readFile(variant.path))
	return true
}

async function handle(
	outDir: string,
	request: IncomingMessage,
	response: ServerResponse,
	next: (error?: unknown) => void,
): Promise<void> {
	if (request.method !== "GET" && request.method !== "HEAD") {
		next()
		return
	}
	try {
		if (!(await respond(outDir, request, response))) {
			next()
		}
	} catch (error) {
		next(error)
	}
}

export function servePages(server: PreviewServer, outDir: string): void {
	server.middlewares.use((request, response, next) => {
		void handle(outDir, request, response, next)
	})
}

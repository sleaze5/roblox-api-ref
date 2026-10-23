import { readFile, stat } from "node:fs/promises"
import type { IncomingMessage, ServerResponse } from "node:http"
import { extname, join, normalize } from "node:path"
import type { PreviewServer } from "vite-plus"
import { negotiate, type Assets } from "../src/edge/negotiate.ts"

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

async function staticResponse(outDir: string, request: Request): Promise<Response> {
	const file = await resolveFile(
		outDir,
		decodeURIComponent(new URL(request.url).pathname),
	)
	if (file === undefined) {
		return new Response("Not Found\n", { status: 404 })
	}
	const variant = await encodedVariant(
		file.path,
		request.headers.get("Accept-Encoding") ?? "",
	)
	const headers = new Headers({
		"Content-Type": contentTypes[extname(file.path)] ?? "application/octet-stream",
		"Vary": "Accept-Encoding",
	})
	if (variant.encoding !== undefined) {
		headers.set("Content-Encoding", variant.encoding)
	}
	const body = request.method === "HEAD" ? null : await readFile(variant.path)
	return new Response(body, { status: file.status, headers })
}

function webRequest(request: IncomingMessage): Request {
	const headers = new Headers()
	for (const [name, value] of Object.entries(request.headers)) {
		if (typeof value === "string") {
			headers.set(name, value)
		}
	}
	const host = request.headers.host ?? "localhost"
	return new Request(new URL(request.url ?? "/", `http://${host}`), {
		method: request.method ?? "GET",
		headers,
	})
}

async function respond(
	outDir: string,
	request: IncomingMessage,
	response: ServerResponse,
): Promise<void> {
	const assets: Assets = { fetch: (asset) => staticResponse(outDir, asset) }
	const page = webRequest(request)
	const result = await negotiate({
		request: page,
		next: () => assets.fetch(page),
		assets,
	})
	response.statusCode = result.status
	result.headers.forEach((value, name) => {
		response.setHeader(name, value)
	})
	response.end(Buffer.from(await result.arrayBuffer()))
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
		await respond(outDir, request, response)
	} catch (error) {
		next(error)
	}
}

export function servePages(server: PreviewServer, outDir: string): void {
	server.middlewares.use((request, response, next) => {
		void handle(outDir, request, response, next)
	})
}

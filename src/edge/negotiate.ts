import { preferredType, type MediaType } from "./accept.ts"
import { isUnchanged, notModified } from "./conditional.ts"
import { linkMarkdown, stampLastModified, varyOnAccept } from "./headers.ts"
import { markdownPath, notFoundMarkdownPath } from "./markdown-path.ts"

export type Assets = {
	fetch: (request: Request) => Promise<Response>
}

export type NegotiationContext = {
	request: Request
	next: () => Promise<Response>
	assets: Assets
	lastModified: string
}

const pageTypes: readonly MediaType[] = ["text/html", "text/markdown"]

const markdownType = "text/markdown; charset=utf-8"

function notAcceptable(): Response {
	return new Response(
		"Not Acceptable\n\nThis page is available as:\n- text/html\n- text/markdown\n",
		{
			status: 406,
			headers: {
				"Content-Type": "text/plain; charset=utf-8",
				"Cache-Control": "no-store",
			},
		},
	)
}

function mutable(response: Response): Response {
	return new Response(response.body, response)
}

function assetRequest(request: Request, path: string, headers?: Headers): Request {
	return new Request(new URL(path, request.url), {
		method: request.method,
		headers: headers ?? new Headers(),
	})
}

async function markdownResponse(
	context: NegotiationContext,
	path: string,
): Promise<Response> {
	const { request, assets } = context
	const page = await assets.fetch(assetRequest(request, path, request.headers))
	if (page.ok || page.status === 304) {
		const markdown = mutable(page)
		markdown.headers.set("Content-Type", markdownType)
		return markdown
	}
	const missing = await assets.fetch(assetRequest(request, notFoundMarkdownPath))
	const notFound = new Response(missing.body, {
		status: 404,
		headers: missing.headers,
	})
	notFound.headers.set("Content-Type", markdownType)
	notFound.headers.set("Cache-Control", "no-store")
	notFound.headers.delete("ETag")
	return notFound
}

function current(page: Response, request: Request, lastModified: string): Response {
	if (isUnchanged(request, lastModified)) {
		void page.body?.cancel()
		return notModified(lastModified)
	}
	return stampLastModified(page, lastModified)
}

export async function negotiate(context: NegotiationContext): Promise<Response> {
	const { request, lastModified } = context
	const path = markdownPath(new URL(request.url).pathname)
	if (path === undefined || (request.method !== "GET" && request.method !== "HEAD")) {
		return context.next()
	}
	const type = preferredType(request.headers.get("Accept"), pageTypes)
	if (type === undefined) {
		return varyOnAccept(notAcceptable())
	}
	if (type === "text/markdown") {
		const markdown = await markdownResponse(context, path)
		return varyOnAccept(
			markdown.ok ? current(markdown, request, lastModified) : markdown,
		)
	}
	const page = mutable(await context.next())
	return varyOnAccept(
		page.ok ? current(linkMarkdown(page, path), request, lastModified) : page,
	)
}

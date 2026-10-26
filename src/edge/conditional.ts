export function isUnchanged(request: Request, lastModified: string): boolean {
	const since = request.headers.get("If-Modified-Since")
	return (
		since !== null &&
		!request.headers.has("If-None-Match") &&
		Date.parse(since) >= Date.parse(lastModified)
	)
}

export function notModified(lastModified: string): Response {
	return new Response(null, {
		status: 304,
		headers: {
			"Cache-Control": "public, max-age=0, must-revalidate",
			"Last-Modified": lastModified,
		},
	})
}

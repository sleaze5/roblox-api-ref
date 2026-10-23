export function varyOnAccept(response: Response): Response {
	const vary = response.headers.get("Vary")
	const fields = vary?.split(",").map((field) => field.trim().toLowerCase()) ?? []
	if (vary === null || vary.trim() === "") {
		response.headers.set("Vary", "Accept")
	} else if (!fields.includes("accept") && !fields.includes("*")) {
		response.headers.set("Vary", `${vary}, Accept`)
	}
	return response
}

export function linkMarkdown(response: Response, path: string): Response {
	const link = `<${path}>; rel="alternate"; type="text/markdown"`
	const existing = response.headers.get("Link")
	response.headers.set("Link", existing === null ? link : `${existing}, ${link}`)
	return response
}

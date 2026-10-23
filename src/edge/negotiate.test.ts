import { describe, expect, test } from "vite-plus/test"
import { negotiate } from "./negotiate.ts"

const files: Record<string, { type: string; body: string }> = {
	"/index.html": { type: "text/html; charset=utf-8", body: "<h1>Home</h1>" },
	"/llms.txt": {
		type: "text/plain; charset=utf-8",
		body: "# Roblox API Reference\n",
	},
	"/classes/Part/index.html": {
		type: "text/html; charset=utf-8",
		body: "<h1>Part</h1>",
	},
	"/classes/Part.md": { type: "text/markdown; charset=utf-8", body: "# Part\n" },
	"/404.html": { type: "text/html; charset=utf-8", body: "<h1>Page not found</h1>" },
	"/404.md": { type: "text/markdown; charset=utf-8", body: "# Page not found\n" },
	"/assets/index.js": { type: "text/javascript", body: "export {}" },
}

function serve(asset: Request): Promise<Response> {
	const path = new URL(asset.url).pathname
	const file = files[path.endsWith("/") ? `${path}index.html` : path]
	const found = file ?? files["/404.html"]
	return Promise.resolve(
		new Response(asset.method === "HEAD" ? null : (found?.body ?? ""), {
			status: file === undefined ? 404 : 200,
			headers: { "Content-Type": found?.type ?? "", "Vary": "Accept-Encoding" },
		}),
	)
}

function request(path: string, accept?: string, method = "GET"): Promise<Response> {
	const page = new Request(`https://example.com${path}`, {
		method,
		headers: accept === undefined ? {} : { Accept: accept },
	})
	return negotiate({
		request: page,
		next: () => serve(page),
		assets: { fetch: serve },
	})
}

describe("negotiate", () => {
	test("serves the Markdown version of a page to agents that ask for it", async () => {
		const response = await request("/classes/Part/", "text/markdown")
		expect(response.status).toBe(200)
		expect(response.headers.get("Content-Type")).toBe(
			"text/markdown; charset=utf-8",
		)
		expect(response.headers.get("Vary")).toBe("Accept-Encoding, Accept")
		expect(await response.text()).toBe("# Part\n")
	})

	test("serves llms.txt as Markdown for the home page", async () => {
		const response = await request("/", "text/markdown, text/html;q=0.9")
		expect(response.headers.get("Content-Type")).toBe(
			"text/markdown; charset=utf-8",
		)
		expect(await response.text()).toBe("# Roblox API Reference\n")
	})

	test.each([undefined, "text/html", "*/*"])(
		"serves HTML for Accept %s and links to the Markdown version",
		async (accept) => {
			const response = await request("/classes/Part/", accept)
			expect(response.status).toBe(200)
			expect(response.headers.get("Content-Type")).toBe(
				"text/html; charset=utf-8",
			)
			expect(response.headers.get("Vary")).toBe("Accept-Encoding, Accept")
			expect(response.headers.get("Link")).toBe(
				'</classes/Part.md>; rel="alternate"; type="text/markdown"',
			)
			expect(await response.text()).toBe("<h1>Part</h1>")
		},
	)

	test("answers a missing page with a Markdown 404 for agents", async () => {
		const response = await request("/classes/Nope/", "text/markdown")
		expect(response.status).toBe(404)
		expect(response.headers.get("Content-Type")).toBe(
			"text/markdown; charset=utf-8",
		)
		expect(response.headers.get("Cache-Control")).toBe("no-store")
		expect(response.headers.get("Vary")).toBe("Accept-Encoding, Accept")
		expect(await response.text()).toBe("# Page not found\n")
	})

	test("answers a missing page with the HTML 404 for browsers", async () => {
		const response = await request("/classes/Nope/", "text/html")
		expect(response.status).toBe(404)
		expect(response.headers.get("Link")).toBeNull()
		expect(response.headers.get("Vary")).toBe("Accept-Encoding, Accept")
		expect(await response.text()).toBe("<h1>Page not found</h1>")
	})

	test("answers 406 when no representation is acceptable", async () => {
		const response = await request("/classes/Part/", "application/pdf")
		expect(response.status).toBe(406)
		expect(response.headers.get("Vary")).toBe("Accept")
		expect(await response.text()).toContain("text/markdown")
	})

	test("sends headers without a body for HEAD", async () => {
		const response = await request("/classes/Part/", "text/markdown", "HEAD")
		expect(response.status).toBe(200)
		expect(response.headers.get("Content-Type")).toBe(
			"text/markdown; charset=utf-8",
		)
		expect(await response.text()).toBe("")
	})

	test("passes files and other methods through untouched", async () => {
		const asset = await request("/assets/index.js", "text/markdown")
		expect(asset.headers.get("Vary")).toBe("Accept-Encoding")
		expect(await asset.text()).toBe("export {}")
		const post = await request("/classes/Part/", "text/markdown", "POST")
		expect(post.headers.get("Content-Type")).toBe("text/html; charset=utf-8")
	})
})

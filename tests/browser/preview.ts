import { spawn, type ChildProcess } from "node:child_process"
import { chromium, type Browser, type Page } from "playwright"

const port = 4317

export const origin = `http://127.0.0.1:${port}`

let server: ChildProcess | undefined
let browser: Browser | undefined

async function isUp(): Promise<boolean> {
	try {
		return (await fetch(`${origin}/`)).ok
	} catch {
		return false
	}
}

async function waitForServer(attempts = 100): Promise<void> {
	if (await isUp()) {
		return
	}
	if (attempts === 0) {
		throw new Error(`The preview server did not start on ${origin}.`)
	}
	await Bun.sleep(100)
	await waitForServer(attempts - 1)
}

/** Serves the built site with the same Markdown negotiation as Cloudflare. */
export async function startPreview(): Promise<void> {
	server = spawn("bun", ["vp", "preview", "--port", String(port), "--strictPort"], {
		stdio: "ignore",
	})
	await waitForServer()
	browser = await chromium.launch(
		process.env["CHROMIUM_PATH"] === undefined
			? {}
			: { executablePath: process.env["CHROMIUM_PATH"] },
	)
}

export async function stopPreview(): Promise<void> {
	await browser?.close()
	server?.kill()
}

export type CheckedPage = Page & { problems: string[] }

/** A new page that records console errors, warnings, and uncaught exceptions. */
export async function openPage(path: string): Promise<CheckedPage> {
	if (browser === undefined) {
		throw new Error("Call startPreview first.")
	}
	const page = (await browser.newPage()) as CheckedPage
	page.problems = []
	page.on("console", (message) => {
		if (message.type() === "error" || message.type() === "warning") {
			page.problems.push(message.text())
		}
	})
	page.on("pageerror", (error) => page.problems.push(String(error)))
	await page.goto(`${origin}${path}`)
	return page
}

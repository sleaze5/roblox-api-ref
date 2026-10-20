import { readFile, writeFile } from "node:fs/promises"
import { parentPort } from "node:worker_threads"
import { writeCompressed } from "./compress.ts"
import { minifyHtml } from "./minify-html.ts"
import type { OutputJob } from "./output-job.ts"

async function run(job: OutputJob): Promise<void> {
	if (job.html === undefined) {
		await writeCompressed(job.file, await readFile(job.file))
		return
	}
	const data = Buffer.from(minifyHtml(job.html))
	await writeFile(job.file, data)
	await writeCompressed(job.file, data)
}

parentPort?.on("message", (job: OutputJob) => {
	run(job).then(
		() => parentPort?.postMessage({ id: job.id }),
		(error: unknown) =>
			parentPort?.postMessage({ id: job.id, error: String(error) }),
	)
})

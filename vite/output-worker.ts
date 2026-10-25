import { writeFile } from "node:fs/promises"
import { parentPort } from "node:worker_threads"
import { minifyHtml } from "./minify-html.ts"
import type { OutputJob } from "./output-job.ts"

async function run(job: OutputJob): Promise<void> {
	await writeFile(job.file, minifyHtml(job.html))
}

let queue = Promise.resolve()

parentPort?.on("message", (job: OutputJob) => {
	queue = queue.then(() =>
		run(job).then(
			() => parentPort?.postMessage({ id: job.id }),
			(error: unknown) =>
				parentPort?.postMessage({ id: job.id, error: String(error) }),
		),
	)
})

import { availableParallelism } from "node:os"
import { Worker } from "node:worker_threads"
import type { OutputJob, OutputResult } from "./output-job.ts"

type Pending = {
	resolve: () => void
	reject: (error: Error) => void
}

export class OutputPool {
	private readonly workers: Worker[]
	private readonly pending = new Map<number, Pending>()
	private nextId = 0

	constructor(workerFile: string) {
		this.workers = Array.from({ length: availableParallelism() }, () => {
			const worker = new Worker(workerFile)
			worker.on("message", (result: OutputResult) => this.settle(result))
			return worker
		})
	}

	get size(): number {
		return this.workers.length
	}

	private settle(result: OutputResult): void {
		const pending = this.pending.get(result.id)
		this.pending.delete(result.id)
		if (result.error === undefined) {
			pending?.resolve()
		} else {
			pending?.reject(new Error(result.error))
		}
	}

	run(job: Omit<OutputJob, "id">): Promise<void> {
		const id = this.nextId++
		const worker = this.workers[id % this.workers.length]
		return new Promise((resolve, reject) => {
			this.pending.set(id, { resolve, reject })
			worker?.postMessage({ ...job, id })
		})
	}

	async close(): Promise<void> {
		await Promise.all(this.workers.map((worker) => worker.terminate()))
	}
}

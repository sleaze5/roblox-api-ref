import { spawn, type ChildProcessByStdio } from "node:child_process"
import { createInterface } from "node:readline"
import type { Readable, Writable } from "node:stream"

type DevProcess = ChildProcessByStdio<Writable, Readable, null>

export function runRapi(root: string, args: string[]): Promise<void> {
	return new Promise((resolve, reject) => {
		const child = spawn("cargo", ["rapi", ...args], { cwd: root, stdio: "inherit" })
		child.on("error", reject)
		child.on("exit", (code) =>
			code === 0
				? resolve()
				: reject(
						new Error(`cargo rapi ${args.join(" ")} exited with ${code}.`),
					),
		)
	})
}

export type DevPage = {
	status: number
	html: string
}

export class DevRenderer {
	readonly #root: string
	#child: DevProcess | undefined
	#waiting: ((line: string) => void)[] = []
	#queue: Promise<unknown> = Promise.resolve()

	constructor(root: string) {
		this.#root = root
	}

	#start(): DevProcess {
		const child = spawn("cargo", ["rapi", "dev"], {
			cwd: this.#root,
			stdio: ["pipe", "pipe", "inherit"],
		})
		createInterface({ input: child.stdout }).on("line", (line) => {
			this.#waiting.shift()?.(line)
		})
		child.on("exit", () => {
			if (this.#child === child) {
				this.#child = undefined
			}
			for (const reject of this.#waiting.splice(0)) {
				reject(JSON.stringify({ status: 500, html: "rapi dev exited." }))
			}
		})
		return child
	}

	render(path: string, template: string): Promise<DevPage> {
		const next = this.#queue.then(
			() =>
				new Promise<DevPage>((resolve) => {
					this.#child ??= this.#start()
					this.#waiting.push((line) => resolve(JSON.parse(line) as DevPage))
					this.#child.stdin.write(`${JSON.stringify({ path, template })}\n`)
				}),
		)
		this.#queue = next.catch(() => undefined)
		return next
	}

	restart(): void {
		this.#child?.kill()
		this.#child = undefined
	}
}

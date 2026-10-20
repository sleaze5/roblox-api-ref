const writeDelay = 300

const pending = new Map<string, string | undefined>()
let timer: ReturnType<typeof setTimeout> | undefined

export function readParam(name: string): string | undefined {
	return new URLSearchParams(window.location.search).get(name) ?? undefined
}

export function flushParams(): void {
	clearTimeout(timer)
	timer = undefined
	if (pending.size === 0) {
		return
	}
	const url = new URL(window.location.href)
	for (const [name, value] of pending) {
		if (value === undefined) {
			url.searchParams.delete(name)
		} else {
			url.searchParams.set(name, value)
		}
	}
	pending.clear()
	url.search = url.search.replaceAll("%2C", ",")
	if (url.href !== window.location.href) {
		history.replaceState(history.state, "", url)
	}
}

export function writeParams(values: Record<string, string | undefined>): void {
	for (const [name, value] of Object.entries(values)) {
		pending.set(name, value)
	}
	clearTimeout(timer)
	timer = setTimeout(flushParams, writeDelay)
}

window.addEventListener("pagehide", flushParams)

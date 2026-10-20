export type RangeReader = {
	size: number
	read(start: number, end: number): Promise<Uint8Array>
}

export function memoryRangeReader(bytes: Uint8Array): RangeReader {
	return {
		size: bytes.length,
		read: async (start, end) => bytes.subarray(start, end),
	}
}

async function contentLength(url: string): Promise<number> {
	const response = await fetch(url, { method: "HEAD" })
	if (!response.ok) {
		throw new Error(`Failed to read ${url}: HTTP ${response.status}`)
	}
	if (response.headers.get("accept-ranges") !== "bytes") {
		throw new Error(`${url} does not support byte range requests.`)
	}
	const size = Number(response.headers.get("content-length"))
	if (!Number.isSafeInteger(size) || size <= 0) {
		throw new Error(`${url} did not report its size.`)
	}
	return size
}

export async function httpRangeReader(url: string): Promise<RangeReader> {
	return {
		size: await contentLength(url),
		read: async (start, end) => {
			const response = await fetch(url, {
				headers: { Range: `bytes=${start}-${end - 1}` },
			})
			if (response.status !== 206) {
				throw new Error(
					`Expected a partial response from ${url}, got HTTP ${response.status}`,
				)
			}
			const bytes = new Uint8Array(await response.arrayBuffer())
			if (bytes.length !== end - start) {
				throw new Error(
					`Expected ${end - start} bytes from ${url}, got ${bytes.length}`,
				)
			}
			return bytes
		},
	}
}

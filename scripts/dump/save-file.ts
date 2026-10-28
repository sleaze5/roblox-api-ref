import { rename } from "node:fs/promises"

export async function saveFile(
	path: string,
	data: Blob | Uint8Array | string,
): Promise<void> {
	const partialPath = `${path}.partial`
	await Bun.write(partialPath, data)
	await rename(partialPath, path)
}

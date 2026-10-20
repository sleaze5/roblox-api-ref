import { readdir, writeFile } from "node:fs/promises"
import { extname, join } from "node:path"
import { promisify } from "node:util"
import { brotliCompress, constants, gzip, zstdCompress } from "node:zlib"

const compressibleExtensions = new Set([
	".html",
	".js",
	".css",
	".json",
	".svg",
	".xml",
	".txt",
])

const brotli = promisify(brotliCompress)
const gzipAsync = promisify(gzip)
const zstd = promisify(zstdCompress)

type Encoder = {
	extension: string
	encode: (data: Buffer) => Promise<Buffer>
}

const encoders: Encoder[] = [
	{
		extension: ".br",
		encode: (data) =>
			brotli(data, {
				params: {
					[constants.BROTLI_PARAM_MODE]: constants.BROTLI_MODE_TEXT,
					[constants.BROTLI_PARAM_QUALITY]: constants.BROTLI_MAX_QUALITY,
					[constants.BROTLI_PARAM_LGWIN]: constants.BROTLI_MAX_WINDOW_BITS,
					[constants.BROTLI_PARAM_SIZE_HINT]: data.length,
				},
			}),
	},
	{
		extension: ".gz",
		encode: (data) =>
			gzipAsync(data, {
				level: constants.Z_BEST_COMPRESSION,
				memLevel: 9,
				windowBits: 15,
			}),
	},
	{
		extension: ".zst",
		encode: (data) =>
			zstd(data, {
				params: {
					[constants.ZSTD_c_compressionLevel]: 22,
					[constants.ZSTD_c_checksumFlag]: 0,
				},
			}),
	},
]

export function isCompressible(file: string): boolean {
	return compressibleExtensions.has(extname(file))
}

export async function compressibleFiles(directory: string): Promise<string[]> {
	const entries = await readdir(directory, { recursive: true, withFileTypes: true })
	return entries
		.filter((entry) => entry.isFile() && isCompressible(entry.name))
		.map((entry) => join(entry.parentPath, entry.name))
}

export async function writeCompressed(file: string, data: Buffer): Promise<void> {
	await Promise.all(
		encoders.map(async (encoder) => {
			const encoded = await encoder.encode(data)
			if (encoded.length < data.length) {
				await writeFile(`${file}${encoder.extension}`, encoded)
			}
		}),
	)
}

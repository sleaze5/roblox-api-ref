import { join } from "node:path"
import subsetFont from "subset-font"
import { generatedDir, rootDir } from "./paths.ts"

type FontSource = {
	family: string
	source: string
	output: string
}

const fonts: FontSource[] = [
	{ family: "Geist", source: "Geist-Variable.woff2", output: "Geist.woff2" },
	{
		family: "Geist Mono",
		source: "GeistMono-Variable.woff2",
		output: "GeistMono.woff2",
	},
]

const weights = { min: 400, max: 600 }

const codePointRanges: [number, number][] = [
	[0x20, 0x7e],
	[0xa0, 0xff],
	[0x2013, 0x2014],
	[0x2018, 0x2019],
	[0x201c, 0x201d],
	[0x2022, 0x2022],
	[0x2026, 0x2026],
	[0x203a, 0x203a],
	[0x2190, 0x2193],
]

function subsetText(): string {
	let text = ""
	for (const [first, last] of codePointRanges) {
		for (let codePoint = first; codePoint <= last; codePoint++) {
			text += String.fromCodePoint(codePoint)
		}
	}
	return text
}

function hex(codePoint: number): string {
	return codePoint.toString(16).toUpperCase()
}

function unicodeRange(): string {
	return codePointRanges
		.map(([first, last]) =>
			first === last ? `U+${hex(first)}` : `U+${hex(first)}-${hex(last)}`,
		)
		.join(",")
}

function fontFace(font: FontSource): string {
	return `@font-face{font-family:"${font.family}";src:url("./fonts/${font.output}") format("woff2");font-weight:${weights.min} ${weights.max};font-style:normal;font-display:optional;unicode-range:${unicodeRange()}}`
}

async function subsetFonts(): Promise<void> {
	const text = subsetText()
	await Promise.all(
		fonts.map(async (font) => {
			const input = Buffer.from(
				await Bun.file(
					join(rootDir, "src", "assets", "fonts", font.source),
				).arrayBuffer(),
			)
			const output = await subsetFont(input, text, {
				targetFormat: "woff2",
				variationAxes: { wght: weights },
			})
			await Bun.write(join(generatedDir, "fonts", font.output), output)
		}),
	)
	await Bun.write(
		join(generatedDir, "fonts.css"),
		`${fonts.map(fontFace).join("\n")}\n`,
	)
}

await subsetFonts()

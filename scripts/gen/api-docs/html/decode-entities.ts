const namedEntities: Record<string, string> = {
	amp: "&",
	lt: "<",
	gt: ">",
	quot: '"',
	apos: "'",
	nbsp: "\u00a0",
	lsquo: "\u2018",
	rsquo: "\u2019",
	ldquo: "\u201c",
	rdquo: "\u201d",
	mdash: "\u2014",
	ndash: "\u2013",
	times: "\u00d7",
	deg: "\u00b0",
	sup2: "\u00b2",
	sup3: "\u00b3",
	hellip: "\u2026",
	rarr: "\u2192",
	larr: "\u2190",
	le: "\u2264",
	ge: "\u2265",
	plusmn: "\u00b1",
}

const entityPattern = /&(?:#x([0-9a-f]+)|#(\d+)|([a-z][a-z0-9]*));/gi

function codePoint(value: number, entity: string): string {
	return Number.isInteger(value) && value > 0 && value <= 0x10ffff
		? String.fromCodePoint(value)
		: entity
}

export function decodeEntities(text: string): string {
	return text.replaceAll(entityPattern, (entity, hex, decimal, name) => {
		if (hex !== undefined) {
			return codePoint(Number.parseInt(hex, 16), entity)
		}
		if (decimal !== undefined) {
			return codePoint(Number(decimal), entity)
		}
		return namedEntities[String(name)] ?? entity
	})
}

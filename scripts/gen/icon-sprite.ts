const svgPattern = /^<svg[^>]*viewBox="([^"]+)"[^>]*>([\s\S]*)<\/svg>\s*$/

function iconPath(name: string): string {
	return Bun.resolveSync(`@phosphor-icons/core/regular/${name}.svg`, import.meta.dir)
}

export async function readIcon(name: string): Promise<string> {
	return (await Bun.file(iconPath(name)).text()).trim()
}

async function readSymbol(name: string): Promise<string> {
	const match = svgPattern.exec(await readIcon(name))
	if (match === null) {
		throw new Error(`Unexpected SVG format in ${iconPath(name)}`)
	}
	const [, viewBox, body] = match
	return `<symbol id="${name}" viewBox="${viewBox}">${body}</symbol>`
}

export async function buildIconSprite(names: readonly string[]): Promise<string> {
	const symbols = await Promise.all(names.map(readSymbol))
	return `<svg xmlns="http://www.w3.org/2000/svg">${symbols.join("")}</svg>\n`
}

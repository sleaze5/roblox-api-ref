import { isAbsolute, join, relative, sep } from "node:path"

function decodePath(pathname: string): string | undefined {
	try {
		return decodeURIComponent(pathname)
	} catch {
		return undefined
	}
}

export function servedPath(outDir: string, pathname: string): string | undefined {
	const urlPath = decodePath(pathname)
	if (urlPath === undefined) {
		return undefined
	}
	const target = join(
		outDir,
		urlPath.endsWith("/") ? `${urlPath}index.html` : urlPath,
	)
	const inside = relative(outDir, target)
	if (inside === ".." || inside.startsWith(`..${sep}`) || isAbsolute(inside)) {
		return undefined
	}
	return target
}

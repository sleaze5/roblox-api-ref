export type PackageSpec = {
	name: string
	version: string
	format: (version: string) => string
}

export function parseSpec(name: string, spec: string): PackageSpec {
	const alias = /^npm:(@?[^@]+)@(.+)$/.exec(spec)
	if (alias !== null) {
		const [, target = name, version = ""] = alias
		return { name: target, version, format: (next) => `npm:${target}@${next}` }
	}
	return { name, version: spec, format: (next) => next }
}

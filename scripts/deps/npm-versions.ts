type Packument = {
	versions: Record<string, { deprecated?: string }>
}

export async function fetchVersions(name: string): Promise<string[]> {
	const url = `https://registry.npmjs.org/${name.replace("/", "%2f")}`
	const response = await fetch(url, {
		headers: { Accept: "application/vnd.npm.install-v1+json" },
	})
	if (!response.ok) {
		throw new Error(`Failed to fetch ${url}: HTTP ${response.status}`)
	}
	const { versions } = (await response.json()) as Packument
	return Object.entries(versions)
		.filter(([, metadata]) => metadata.deprecated === undefined)
		.map(([version]) => version)
}

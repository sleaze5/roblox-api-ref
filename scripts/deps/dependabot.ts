type DependabotConfig = {
	updates: {
		"package-ecosystem": string
		"ignore"?: {
			"dependency-name": string
			"update-types"?: string[]
			"versions"?: string[]
		}[]
	}[]
}

export function skippedPackages(yaml: string): Set<string> {
	const config = Bun.YAML.parse(yaml) as DependabotConfig
	const rules =
		config.updates.find((update) => update["package-ecosystem"] === "bun")
			?.ignore ?? []
	return new Set(
		rules
			.filter(
				(rule) =>
					rule["update-types"] === undefined && rule.versions === undefined,
			)
			.map((rule) => rule["dependency-name"]),
	)
}

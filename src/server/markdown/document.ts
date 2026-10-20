export type Fact = [label: string, value: string | undefined]

export function factList(facts: Fact[]): string {
	return facts
		.filter(
			(fact): fact is [string, string] => fact[1] !== undefined && fact[1] !== "",
		)
		.map(([label, value]) => `- ${label}: ${value}`)
		.join("\n")
}

export function joinBlocks(blocks: (string | undefined)[]): string {
	return `${blocks.filter((block) => block !== undefined && block !== "").join("\n\n")}\n`
}

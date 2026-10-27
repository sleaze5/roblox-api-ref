export type SearchModule = {
	showSearch: (query?: string) => void
}

export type Recovery = {
	load: () => Promise<SearchModule>
	canReload: boolean
	reload: (query: string | undefined) => void
	report: () => void
}

export async function showOrRecover(
	query: string | undefined,
	recovery: Recovery,
): Promise<void> {
	const search = await recovery.load().catch(() => undefined)
	if (search !== undefined) {
		search.showSearch(query)
	} else if (recovery.canReload) {
		recovery.reload(query)
	} else {
		recovery.report()
	}
}

export type ApiDocsParam = {
	name: string
	documentation: string
}

export type ApiDocsEntry = {
	documentation?: string
	params?: ApiDocsParam[]
	returns?: string[]
}

export type ApiDocs = Record<string, ApiDocsEntry | undefined>

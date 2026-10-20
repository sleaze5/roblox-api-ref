import type { Doc } from "../../../src/api/doc-types.ts"
import type { ApiDocs } from "./api-docs-types.ts"
import type { ApiNames } from "./doc/api-names.ts"
import { resolveCode, type CodeContext } from "./doc/resolve-code.ts"
import { resolveHref } from "./doc/resolve-href.ts"
import { toDoc, type LinkResolver } from "./doc/to-doc.ts"
import { parseHtml } from "./html/parse-html.ts"

export type Describe = (key: string | undefined) => Doc | undefined

export function describer(
	names: ApiNames,
	docs: ApiDocs,
	context: CodeContext,
): Describe {
	const links: LinkResolver = {
		href: (href) => resolveHref(names, href),
		code: (text) => resolveCode(names, text, context),
	}
	return (key) => {
		const html = key === undefined ? undefined : docs[key]?.documentation
		return html === undefined ? undefined : toDoc(parseHtml(html), links)
	}
}

import { summaryText } from "../client/filters/summary-text.ts"

const listFormPattern = /<form\b[^>]*\sdata-list-filter="([^"]+)"[^>]*>/g
const memberFormPattern = /<form\b[^>]*\sdata-member-filter(?:="")?(?=[\s>])[^>]*>/g
const summaryPattern =
	/(<output\b[^>]*\sdata-filter-summary(?:="")?(?=[\s>])[^>]*>)(<\/output>)/

function count(html: string, pattern: RegExp): number {
	return html.match(pattern)?.length ?? 0
}

function elementById(html: string, id: string): string | undefined {
	const open = new RegExp(`<([a-z]+)\\b[^>]*\\sid="${id}"[^>]*>`).exec(html)
	if (open === null) {
		return undefined
	}
	const tag = open[1] ?? ""
	const pattern = new RegExp(`<${tag}[\\s>]|</${tag}>`, "g")
	pattern.lastIndex = open.index + open[0].length
	let depth = 1
	for (let match = pattern.exec(html); match !== null; match = pattern.exec(html)) {
		depth += match[0].startsWith("</") ? -1 : 1
		if (depth === 0) {
			return html.slice(open.index, match.index)
		}
	}
	return undefined
}

function listTotal(list: string): number {
	const open = list.slice(0, list.indexOf(">") + 1)
	if (/\sdata-tree[\s=>]/.test(open)) {
		return count(list, /<li[\s>]/g)
	}
	if (/\sdata-changes[\s=>]/.test(open)) {
		return count(list, /\sdata-change(?=[\s=>])/g)
	}
	return count(list, /\sdata-name="/g)
}

function fillSummary(form: string, text: string): string {
	return form.replace(summaryPattern, `$1${text}$2`)
}

function formEnd(html: string, start: number): number {
	const end = html.indexOf("</form>", start)
	return end === -1 ? html.length : end
}

function replaceForms(
	html: string,
	pattern: RegExp,
	text: (open: RegExpExecArray) => string | undefined,
): string {
	let result = ""
	let last = 0
	for (const open of html.matchAll(pattern)) {
		const end = formEnd(html, open.index)
		const summary = text(open)
		result += html.slice(last, open.index)
		result +=
			summary === undefined
				? html.slice(open.index, end)
				: fillSummary(html.slice(open.index, end), summary)
		last = end
	}
	return result + html.slice(last)
}

export function fillFilterSummaries(html: string): string {
	const withLists = replaceForms(html, listFormPattern, (open) => {
		const list = elementById(html, open[1] ?? "")
		if (list === undefined) {
			return undefined
		}
		const total = listTotal(list)
		const noun = /\sdata-noun="([^"]+)"/.exec(open[0])?.[1]
		return noun === undefined ? summaryText(total, total) : `${total} ${noun}`
	})
	const members = count(withLists, /\sdata-member(?=[\s=>])/g)
	return replaceForms(withLists, memberFormPattern, () =>
		summaryText(members, members),
	)
}

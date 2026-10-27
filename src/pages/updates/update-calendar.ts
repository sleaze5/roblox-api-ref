import { formatMonth } from "../../api/update-text.ts"
import type { Update } from "../../api/update-types.ts"

export type UpdateMonth = {
	title: string
	updates: Update[]
}

export type UpdateYear = {
	year: string
	count: number
	months: UpdateMonth[]
}

export function updateCalendar(updates: Update[]): UpdateYear[] {
	const years: UpdateYear[] = []
	for (const update of updates) {
		const year = update.published.slice(0, 4)
		const month = formatMonth(update.published)
		let yearGroup = years.at(-1)
		if (yearGroup?.year !== year) {
			yearGroup = { year, count: 0, months: [] }
			years.push(yearGroup)
		}
		let monthGroup = yearGroup.months.at(-1)
		if (monthGroup?.title !== month) {
			monthGroup = { title: month, updates: [] }
			yearGroup.months.push(monthGroup)
		}
		monthGroup.updates.push(update)
		yearGroup.count++
	}
	return years
}

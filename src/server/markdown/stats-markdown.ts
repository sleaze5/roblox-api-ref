import { updateHref } from "../../api/routes.ts"
import { statRefLabel } from "../../api/stat-ref-label.ts"
import {
	countLabels,
	deprecatedTotal,
	removedNote,
	statsLead,
	trendsNote,
} from "../../api/stats-text.ts"
import type { ChangeTally, RecordGroup, Stats, Tallies } from "../../api/stats-types.ts"
import type { Release } from "../../api/release.ts"
import { formatDate } from "../../api/update-text.ts"
import { factList, joinBlocks } from "./document.ts"
import { inlineCode, link, tableCell } from "./escape.ts"
import { markdownUrl } from "./markdown-href.ts"

function table(headings: string[], rows: string[][]): string {
	return [
		`| ${headings.join(" | ")} |`,
		`| ${headings.map(() => "---").join(" | ")} |`,
		...rows.map((row) => `| ${row.map(tableCell).join(" | ")} |`),
	].join("\n")
}

function countTable(stats: Stats): string {
	const current = stats.counts.reduce((sum, row) => sum + row.current, 0)
	const removed = stats.counts.reduce((sum, row) => sum + row.removed, 0)
	return table(
		["Kind", "Current", "Removed", "Total"],
		[
			...stats.counts.map((row) => [
				countLabels[row.kind],
				String(row.current),
				String(row.removed),
				String(row.current + row.removed),
			]),
			["All", String(current), String(removed), String(current + removed)],
		],
	)
}

function recordBlocks(group: RecordGroup): string[] {
	return [
		`## ${group.title}`,
		table(
			["Statistic", "Value", "Items"],
			group.records.map((record) => [
				record.label,
				String(record.value),
				record.items.map((item) => inlineCode(statRefLabel(item))).join(", "),
			]),
		),
	]
}

function tallyText(tally: ChangeTally, changes: boolean): string {
	const parts = [
		tally.added > 0 ? `+${tally.added}` : undefined,
		tally.removed > 0 ? `−${tally.removed}` : undefined,
		changes && tally.changed > 0 ? `~${tally.changed}` : undefined,
	].filter((part) => part !== undefined)
	return parts.length === 0 ? "0" : parts.join(" ")
}

function tallyCells(tallies: Tallies): string[] {
	return [
		tallyText(tallies.Class, false),
		tallyText(tallies.Member, true),
		tallyText(tallies.Enum, false),
		tallyText(tallies.EnumItem, true),
		String(deprecatedTotal(tallies)),
	]
}

const tallyHeadings = ["Classes", "Members", "Enums", "Enum items", "Deprecated"]

export function statsMarkdown(release: Release, stats: Stats): string {
	return joinBlocks([
		"# Statistics",
		statsLead(release),
		factList([
			["Version", release.version],
			["Tracked since", stats.historyStart],
			["Updates", String(stats.updates.length)],
		]),
		"## Counts",
		removedNote(stats.historyStart),
		countTable(stats),
		...stats.records.flatMap(recordBlocks),
		"## API size",
		table(
			["Update", "Published", "Classes", "Enums"],
			stats.updates
				.toReversed()
				.map((trend) => [
					trend.version,
					formatDate(trend.published),
					String(trend.classes),
					String(trend.enums),
				]),
		),
		"## Changes by year",
		`${trendsNote} + means added, − removed, and ~ changed.`,
		table(
			["Year", "Updates", ...tallyHeadings],
			stats.years
				.toReversed()
				.map((year) =>
					[year.year, String(year.updates)].concat(tallyCells(year.tallies)),
				),
		),
		"## Changes by update",
		table(
			["Update", "Published", ...tallyHeadings],
			stats.updates
				.toReversed()
				.map((trend) =>
					[
						link(trend.version, markdownUrl(updateHref(trend.version))),
						formatDate(trend.published),
					].concat(tallyCells(trend.tallies)),
				),
		),
	])
}

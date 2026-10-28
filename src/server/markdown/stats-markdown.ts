import { updateHref } from "../../api/routes.ts"
import { statRefLabel } from "../../api/stat-ref-label.ts"
import {
	changeTotal,
	countLabels,
	monthLabel,
	monthsNote,
	deprecatedTotal,
	removedNote,
	statsLead,
	trendsNote,
} from "../../api/stats-text.ts"
import type { ChangeTally, RecordGroup, Stats, Tallies } from "../../api/stats-types.ts"
import type { Release } from "../../api/release.ts"
import { formatDate } from "../../api/update-text.ts"
import { factList, joinBlocks } from "./document.ts"
import { inlineCode, link } from "./escape.ts"
import {
	comebackMarkdown,
	extraMarkdown,
	markdownTable,
} from "./stats-extra-markdown.ts"
import { markdownUrl } from "./markdown-href.ts"

function countTable(stats: Stats): string {
	const current = stats.counts.reduce((sum, row) => sum + row.current, 0)
	const removed = stats.counts.reduce((sum, row) => sum + row.removed, 0)
	return markdownTable(
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

function recordBlocks(group: RecordGroup, stats: Stats): string[] {
	return [
		`## ${group.title}`,
		markdownTable(
			["Statistic", "Value", "Items"],
			group.records.map((record) => [
				record.label,
				record.unit === undefined
					? String(record.value)
					: `${record.value} ${record.unit}`,
				record.items.map((item) => inlineCode(statRefLabel(item))).join(", "),
			]),
		),
		...(group.id === "comeback-records" ? comebackMarkdown(stats) : []),
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
		...stats.records.flatMap((group) => recordBlocks(group, stats)),
		...extraMarkdown(stats),
		"## API size",
		markdownTable(
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
		markdownTable(
			["Year", "Updates", ...tallyHeadings],
			stats.years
				.toReversed()
				.map((year) =>
					[year.period, String(year.updates)].concat(
						tallyCells(year.tallies),
					),
				),
		),
		"## Changes by month",
		monthsNote,
		markdownTable(
			["Month", "Updates", "Changes", ...tallyHeadings],
			stats.months.map((month) =>
				[
					monthLabel(month.period),
					String(month.updates),
					String(changeTotal(month.tallies)),
				].concat(tallyCells(month.tallies)),
			),
		),
		"## Changes by update",
		markdownTable(
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

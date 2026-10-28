import { accessTierLabels } from "../../api/access-tiers.ts"
import {
	accessLead,
	capabilitiesLead,
	comebacksLead,
	coverageLabels,
	docsLead,
	growthLead,
	percentText,
	tagsLead,
} from "../../api/stats-text.ts"
import type { Stats, TagTrend, UpdateEvent } from "../../api/stats-types.ts"
import { formatDate } from "../../api/update-text.ts"
import { inlineCode, tableCell } from "./escape.ts"

export function markdownTable(headings: string[], rows: string[][]): string {
	return [
		`| ${headings.join(" | ")} |`,
		`| ${headings.map(() => "---").join(" | ")} |`,
		...rows.map((row) => `| ${row.map(tableCell).join(" | ")} |`),
	].join("\n")
}

function updateText(update: UpdateEvent | undefined): string {
	return update === undefined
		? "Not recorded"
		: `${update.version} (${formatDate(update.published)})`
}

export function comebackMarkdown(stats: Stats): string[] {
	return [
		comebacksLead,
		markdownTable(
			["Removed in", "Added back in", "Items", "Days gone"],
			stats.comebacks.map((batch) => [
				updateText(batch.removed),
				updateText(batch.returned),
				String(batch.count),
				String(batch.days),
			]),
		),
	]
}

function tagTable(label: string, rows: TagTrend[]): string {
	return markdownTable(
		[label, "Current", "Gained", "Lost", "First gained"],
		rows.map((row) => [
			inlineCode(row.name),
			String(row.current),
			String(row.gained),
			String(row.lost),
			updateText(row.first),
		]),
	)
}

export function extraMarkdown(stats: Stats): string[] {
	return [
		"## Access",
		accessLead(stats.access.now),
		markdownTable(
			["Who can use it", "Properties", "Methods", "Events", "Callbacks", "All"],
			stats.access.tiers.map((row) => [
				accessTierLabels[row.tier],
				String(row.kinds.Property),
				String(row.kinds.Method),
				String(row.kinds.Event),
				String(row.kinds.Callback),
				String(
					row.kinds.Property +
						row.kinds.Method +
						row.kinds.Event +
						row.kinds.Callback,
				),
			]),
		),
		markdownTable(
			[
				"Year",
				"Added open",
				"Added restricted",
				"Opened",
				"Restricted",
				"Made safe",
				"Made unsafe",
			],
			stats.access.years
				.toReversed()
				.map((year) => [
					year.period,
					String(year.addedOpen),
					String(year.addedRestricted),
					String(year.opened),
					String(year.restricted),
					String(year.madeSafe),
					String(year.madeUnsafe),
				]),
		),
		"## Tags",
		tagsLead,
		tagTable("Tag", stats.tags),
		"## Capabilities",
		capabilitiesLead,
		tagTable("Capability", stats.capabilities),
		"## Descriptions",
		docsLead,
		markdownTable(
			["Kind", "Described", "Total", "Share"],
			stats.docs.coverage.map((row) => [
				coverageLabels[row.kind],
				String(row.described),
				String(row.total),
				percentText(row.described, row.total),
			]),
		),
		markdownTable(
			["Year", "Added", "Changed", "Removed"],
			stats.docs.years
				.toReversed()
				.map((year) => [
					year.period,
					String(year.added),
					String(year.changed),
					String(year.removed),
				]),
		),
		"## Growth by class",
		growthLead,
		markdownTable(
			["Year", "Classes that grew the most"],
			stats.growth
				.toReversed()
				.map((year) => [
					year.period,
					year.classes
						.map((item) => `${inlineCode(item.name)} +${item.added}`)
						.join(", "),
				]),
		),
	]
}

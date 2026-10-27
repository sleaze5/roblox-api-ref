import { releaseText } from "../../api/release.ts"
import { addedTags } from "../../api/added-tags.ts"
import { inlineTokens, signatureParts } from "../../api/signature.ts"
import { plainTags } from "../../api/tags.ts"
import type { EnumItem, Member } from "../../api/types.ts"
import {
	changeTotal,
	formatDate,
	updateSummary,
	updatesLead,
} from "../../api/update-text.ts"
import type {
	ChangeDetail,
	ChangeEntry,
	DetailValue,
	OwnerChanges,
	OwnerKind,
	Update,
} from "../../api/update-types.ts"
import { updateHref, updatesHref } from "../../api/routes.ts"
import { descriptionLines } from "./description-markdown.ts"
import { factList, joinBlocks } from "./document.ts"
import { escapeText, inlineCode, link } from "./escape.ts"
import { markdownHref } from "./markdown-href.ts"
import { absoluteUrl } from "../../site/origin.ts"
import { groupOwners, ownerGroupTitles } from "../../api/owner-groups.ts"

const ownerTitles = { Class: "Classes", Enum: "Enums" } as const

function valueText(value: DetailValue): string {
	return value.tone === "text" ? value.text : inlineCode(value.text)
}

function detailText(detail: ChangeDetail): string {
	if (detail.kind === "tags") {
		return `${detail.label}: ${plainTags(detail.tags)
			.map((tag) => tag.label)
			.join(", ")}`
	}
	const before = detail.before === undefined ? "none" : valueText(detail.before)
	const after = detail.after === undefined ? "none" : valueText(detail.after)
	return detail.before === undefined
		? `${detail.label}: ${after}`
		: `${detail.label}: ${before} to ${after}`
}

function signatureText(owner: string, value: Member | EnumItem): string {
	if (!("kind" in value)) {
		return `Enum.${owner}.${value.name} = ${value.value}`
	}
	return inlineTokens(signatureParts(value, owner))
		.map((part) => part.text)
		.join("")
}

function withDetails(text: string, details: ChangeDetail[]): string {
	return details.length === 0
		? text
		: `${text}. ${details.map(detailText).join(". ")}`
}

function entryLine(entry: ChangeEntry): string {
	const before = entry.before && signatureText(entry.owner, entry.before)
	const after = entry.after && signatureText(entry.owner, entry.after)
	let text = `${entry.status} ${inlineCode(after ?? before ?? entry.name)}`
	if (before !== undefined && after !== undefined && before !== after) {
		text = `Changed ${inlineCode(before)} to ${inlineCode(after)}`
	}
	const tags = addedTags(entry).map((tag) => tag.label)
	const tagText = tags.length === 0 ? text : `${text}. Tags: ${tags.join(", ")}`
	return [
		`- ${withDetails(tagText, entry.details)}`,
		...descriptionLines(entry.descriptions, "  "),
	].join("\n")
}

function ownerBlock(owner: OwnerChanges): string {
	const kind = owner.kind.toLowerCase()
	const lines = [
		...(owner.status === undefined
			? []
			: [`- ${withDetails(`${owner.status} ${kind}`, owner.details)}`]),
		...descriptionLines(owner.descriptions, ""),
		...owner.entries.map(entryLine),
	]
	return `#### ${escapeText(owner.name)}\n\n${lines.join("\n")}`
}

function countLine(update: Update): string {
	return `${update.counts.Added} added, ${update.counts.Removed} removed, and ${update.counts.Changed} changed`
}

function kindBlocks(update: Update, kind: OwnerKind): string[] {
	const groups = groupOwners(update.owners, kind)
	if (groups.length === 0) {
		return []
	}
	const blocks = [`## ${ownerTitles[kind]}`]
	for (const list of groups) {
		blocks.push(`### ${ownerGroupTitles[list.group]}`)
		blocks.push(...list.owners.map(ownerBlock))
	}
	return blocks
}

export function updateMarkdown(update: Update): string {
	const sections = (["Class", "Enum"] as const).flatMap((kind) =>
		kindBlocks(update, kind),
	)
	return joinBlocks([
		`# Update ${update.version}`,
		`${updateSummary(update)} Roblox published this version on ${formatDate(update.published)}. It has ${changeTotal(update)} changes: ${countLine(update)}. The HTML version of this page is at ${absoluteUrl(updateHref(update.version))}.`,
		factList([
			["Version", releaseText(update)],
			["Compared with", releaseText(update.previous)],
			["Description changes", String(update.descriptionCount)],
		]),
		...sections,
	])
}

export function updateListMarkdown(updates: Update[]): string {
	const lines = updates.map(
		(update) =>
			`- ${link(releaseText(update), absoluteUrl(markdownHref(updateHref(update.version))))}: ${formatDate(update.published)}, compared with ${releaseText(update.previous)}. ${countLine(update)}.`,
	)
	return joinBlocks([
		"# Updates",
		`${updatesLead} The HTML version of this list is at ${absoluteUrl(updatesHref())}.`,
		lines.length === 0 ? "There are no updates yet." : lines.join("\n"),
	])
}

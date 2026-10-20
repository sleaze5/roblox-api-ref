import { compareNames } from "./compare-names.ts"
import {
	groupedMemberTags,
	memberTagGroups,
	memberTagGroupTitles,
	type MemberTagGroup,
} from "./tags.ts"
import type { Member } from "./types.ts"

export type FilterTag = {
	key: string
	label: string
	shortLabel: string
	count: number
}

export type FilterTagGroup = {
	group: MemberTagGroup
	title: string
	tags: FilterTag[]
}

export function tagKey(label: string): string {
	return label
		.toLowerCase()
		.replaceAll(/[^a-z0-9]+/g, "-")
		.replaceAll(/^-|-$/g, "")
}

function shortLabel(group: MemberTagGroup, label: string): string {
	return group === "capability"
		? label
				.replace(/^Capabilities: /, "")
				.replace(/^(Read|Write) capabilities: /, "$1: ")
		: label
}

export function memberTagKeys(member: Member): string[] {
	const groups = groupedMemberTags(member)
	return memberTagGroups.flatMap((group) =>
		groups[group].map((tag) => tagKey(tag.label)),
	)
}

export function filterTagGroups(members: Member[]): FilterTagGroup[] {
	const counts = new Map<MemberTagGroup, Map<string, FilterTag>>(
		memberTagGroups.map((group) => [group, new Map()]),
	)
	for (const member of members) {
		const groups = groupedMemberTags(member)
		for (const group of memberTagGroups) {
			const tags = counts.get(group)
			for (const tag of groups[group]) {
				const key = tagKey(tag.label)
				const entry = tags?.get(key) ?? {
					key,
					label: tag.label,
					shortLabel: shortLabel(group, tag.label),
					count: 0,
				}
				entry.count++
				tags?.set(key, entry)
			}
		}
	}
	return memberTagGroups
		.map((group) => ({
			group,
			title: memberTagGroupTitles[group],
			tags: [...(counts.get(group)?.values() ?? [])].sort((a, b) =>
				compareNames(a.shortLabel, b.shortLabel),
			),
		}))
		.filter((group) => group.tags.length > 0)
}

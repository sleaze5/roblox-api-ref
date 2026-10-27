import { releaseName } from "./release.ts"
import {
	changeStatuses,
	type OwnerChanges,
	type OwnerKind,
	type Update,
} from "./update-types.ts"

const dateFormat = new Intl.DateTimeFormat("en-US", {
	year: "numeric",
	month: "long",
	day: "numeric",
	timeZone: "UTC",
})

const shortDateFormat = new Intl.DateTimeFormat("en-US", {
	month: "short",
	day: "numeric",
	timeZone: "UTC",
})

const monthFormat = new Intl.DateTimeFormat("en-US", {
	month: "long",
	timeZone: "UTC",
})

export const updatesLead =
	"Every change to the Roblox Engine API between the Roblox Studio versions that this site has published, newest first."

export function formatDate(iso: string): string {
	return dateFormat.format(new Date(iso))
}

export function formatShortDate(iso: string): string {
	return shortDateFormat.format(new Date(iso))
}

export function formatMonth(iso: string): string {
	return monthFormat.format(new Date(iso))
}

export function updateSummary(update: Update): string {
	return `Changes to the Roblox Engine API from ${releaseName(update.previous)} to ${releaseName(update)}.`
}

export function changeTotal(update: Update): number {
	return changeStatuses.reduce((total, status) => total + update.counts[status], 0)
}

export function updateAnchor(version: string): string {
	return `update-${version}`
}

export function ownerAnchor(owner: Pick<OwnerChanges, "kind" | "name">): string {
	return `${owner.kind}-${owner.name}`
}

export function changeAnchor(kind: OwnerKind, owner: string, name: string): string {
	return `${ownerAnchor({ kind, name: owner })}.${name}`
}

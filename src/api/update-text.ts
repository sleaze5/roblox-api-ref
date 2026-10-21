import { changeStatuses, type OwnerChanges, type Update } from "./update-types.ts"

const dateFormat = new Intl.DateTimeFormat("en-US", {
	year: "numeric",
	month: "long",
	day: "numeric",
	timeZone: "UTC",
})

export const updatesLead =
	"Every change to the Roblox Engine API between the Studio versions that this site has published, newest first."

export function formatDate(iso: string): string {
	return dateFormat.format(new Date(iso))
}

export function updateSummary(update: Update): string {
	return `Changes to the Roblox Engine API from Studio ${update.previous.version} to ${update.version}.`
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

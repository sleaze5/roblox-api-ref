import { releaseName } from "../api/release.ts"
import { updateHref, updatesHref } from "../api/routes.ts"
import { updateSummary } from "../api/update-text.ts"
import type { Update } from "../api/update-types.ts"
import { siteName } from "../site/brand.ts"
import { absoluteUrl } from "../site/origin.ts"

export const feedPath = "/updates.xml"

const feedLength = 20

function escapeXml(value: string): string {
	return value
		.replaceAll("&", "&amp;")
		.replaceAll("<", "&lt;")
		.replaceAll(">", "&gt;")
		.replaceAll('"', "&quot;")
}

function entry(update: Update): string {
	const url = absoluteUrl(updateHref(update.version))
	const summary = `${updateSummary(update)} ${update.counts.Added} added, ${update.counts.Removed} removed, and ${update.counts.Changed} changed.`
	return `<entry><title>${escapeXml(releaseName(update))}</title><id>${escapeXml(url)}</id><link href="${escapeXml(url)}"/><updated>${update.published}</updated><summary>${escapeXml(summary)}</summary></entry>`
}

export function updatesFeed(updates: Update[]): string {
	const recent = updates.slice(0, feedLength)
	const updated = recent[0]?.published ?? new Date(0).toISOString()
	return `<?xml version="1.0" encoding="UTF-8"?><feed xmlns="http://www.w3.org/2005/Atom"><title>${escapeXml(`${siteName} updates`)}</title><id>${escapeXml(absoluteUrl(feedPath))}</id><link rel="self" href="${escapeXml(absoluteUrl(feedPath))}"/><link href="${escapeXml(absoluteUrl(updatesHref()))}"/><updated>${updated}</updated><author><name>${escapeXml(siteName)}</name></author>${recent.map(entry).join("")}</feed>\n`
}

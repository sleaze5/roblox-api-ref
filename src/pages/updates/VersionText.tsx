import { shownVersion, uploadName, type Release } from "../../api/release.ts"

export function VersionText(props: { release: Release }) {
	const shown = shownVersion(props.release)
	return (
		<span class="font-mono">
			{shown.version} ({uploadName(shown.hash)})
		</span>
	)
}

import { uploadName, type Release } from "../../api/release.ts"

export function VersionText(props: { release: Release }) {
	return (
		<span class="font-mono">
			{props.release.version} ({uploadName(props.release.hash)})
		</span>
	)
}

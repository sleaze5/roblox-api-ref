import { externalLink } from "./external-link.ts"

export function CreatorHubLink(props: { href: string; name: string }) {
	return (
		<a
			href={props.href}
			{...externalLink}
			data-tooltip="Open on Creator Hub"
			aria-label={`Open ${props.name} on Creator Hub (opens in a new tab)`}
			class="docs-link"
		/>
	)
}

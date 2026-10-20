import { classDocsHref } from "../../api/docs-links.ts"
import { CreatorHubLink } from "../CreatorHubLink.tsx"

export function CopyLink(props: { name: string; href: string }) {
	return (
		<a
			href={props.href}
			data-copy-link=""
			data-tooltip="Copy link"
			aria-label={`Copy link to ${props.name}`}
			class="copy-link"
		/>
	)
}

export function DocsLink(props: { name: string; owner: string }) {
	return (
		<CreatorHubLink
			href={classDocsHref(props.owner, props.name)}
			name={props.name}
		/>
	)
}

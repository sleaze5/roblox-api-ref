import { externalLink } from "../../components/external-link.ts"
import { isExternal, type Inline } from "../../site/content/page-content.ts"

function InlinePart(props: { part: Inline }) {
	const part = props.part
	if (typeof part === "string") {
		return part
	}
	if ("code" in part) {
		return <code>{part.code}</code>
	}
	if (!isExternal(part.href)) {
		return <a href={part.href}>{part.text}</a>
	}
	return (
		<a href={part.href} {...externalLink}>
			{part.text}
			<span class="sr-only"> (opens in a new tab)</span>
		</a>
	)
}

export function InlineContent(props: { content: Inline[] }) {
	return (
		<>
			{props.content.map((part) => (
				<InlinePart part={part} />
			))}
		</>
	)
}

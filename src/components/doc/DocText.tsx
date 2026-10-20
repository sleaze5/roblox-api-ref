import type { JSX } from "@solidjs/web"
import type { Doc, DocElement, DocNode } from "../../api/doc-types.ts"
import { externalLink } from "../external-link.ts"
import { docCodeTone, docLinkHref } from "./doc-link.ts"

function DocChildren(props: { nodes: DocNode[] }) {
	return (
		<>
			{props.nodes.map((node) =>
				typeof node === "string" ? node : <DocTag element={node} />,
			)}
		</>
	)
}

function DocAnchor(props: { element: DocElement }) {
	const link = props.element.link
	if (link === undefined) {
		return <DocChildren nodes={props.element.children} />
	}
	return link.kind === "external" ? (
		<a href={link.url} {...externalLink}>
			<DocChildren nodes={props.element.children} />
			<span class="sr-only"> (opens in a new tab)</span>
		</a>
	) : (
		<a href={docLinkHref(link)}>
			<DocChildren nodes={props.element.children} />
		</a>
	)
}

function DocCode(props: { element: DocElement }) {
	const link = props.element.link
	const code = (
		<code>
			<DocChildren nodes={props.element.children} />
		</code>
	)
	return link === undefined ? (
		code
	) : (
		<a href={docLinkHref(link)} data-code={docCodeTone(link)}>
			{code}
		</a>
	)
}

const renderers: Record<DocElement["tag"], (element: DocElement) => JSX.Element> = {
	a: (element) => <DocAnchor element={element} />,
	br: () => <br />,
	code: (element) => <DocCode element={element} />,
	em: (element) => (
		<em>
			<DocChildren nodes={element.children} />
		</em>
	),
	kbd: (element) => (
		<kbd class="kbd">
			<DocChildren nodes={element.children} />
		</kbd>
	),
	li: (element) => (
		<li>
			<DocChildren nodes={element.children} />
		</li>
	),
	pre: (element) => (
		<pre>
			<DocChildren nodes={element.children} />
		</pre>
	),
	strong: (element) => (
		<strong>
			<DocChildren nodes={element.children} />
		</strong>
	),
	sup: (element) => (
		<sup>
			<DocChildren nodes={element.children} />
		</sup>
	),
	ul: (element) => (
		<ul>
			<DocChildren nodes={element.children} />
		</ul>
	),
}

function DocTag(props: { element: DocElement }) {
	return renderers[props.element.tag](props.element)
}

export function DocText(props: { doc: Doc | undefined; class?: string }) {
	return props.doc === undefined ? null : (
		<div class={["doc-text", props.class]}>
			<DocChildren nodes={props.doc} />
		</div>
	)
}

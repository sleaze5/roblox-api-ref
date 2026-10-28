import type { SearchEngine, SearchResult } from "../../search/engine.ts"
import { entryFlags } from "../../search/index-format.ts"
import { resultHref, roleLabels } from "../../search/result-info.ts"
import { Highlight } from "./Highlight.tsx"
import { KindIcon } from "./KindIcon.tsx"
import { activeResult, setActiveResult } from "./search-state.ts"

export function optionId(index: number): string {
	return `search-option-${index}`
}

function Context(props: { engine: SearchEngine; result: SearchResult }) {
	const parent = () => props.engine.parentOf(props.result.entry)
	const isPrimary = () => props.engine.isPrimary(props.result.entry)
	const inheritedFrom = () =>
		props.result.scope !== undefined && props.result.scope !== parent()
			? props.engine.nameOf(parent())
			: undefined
	return (
		<span class="truncate font-mono text-fg-faint">
			{isPrimary()
				? parent() === -1
					? ""
					: props.engine.nameOf(parent())
				: inheritedFrom() === undefined
					? props.engine.nameOf(parent())
					: `inherited from ${inheritedFrom()}`}
		</span>
	)
}

export function ResultOption(props: {
	engine: SearchEngine
	result: SearchResult
	index: number
	word: string
}) {
	const deprecated = () =>
		(props.engine.flagsOf(props.result.entry) & entryFlags.deprecated) !== 0
	return (
		<a
			id={optionId(props.index)}
			role="option"
			aria-selected={activeResult() === props.index ? "true" : "false"}
			href={resultHref(props.engine, props.result)}
			onPointerMove={() => setActiveResult(props.index)}
			class="flex h-8 items-center gap-2 rounded-sm px-2 text-fg-muted aria-selected:bg-hover aria-selected:text-fg">
			<KindIcon
				kind={props.engine.kindOf(props.result.entry)}
				classIcon={props.engine.classIconOf(props.result.entry)}
				class="text-fg-faint"
			/>
			<span
				class={[
					"shrink-0 font-mono",
					deprecated() ? "text-fg-muted" : "text-fg",
				]}>
				<Highlight
					name={props.engine.nameOf(props.result.entry)}
					word={props.word}
				/>
			</span>
			<Context engine={props.engine} result={props.result} />
			<span class="ml-auto flex shrink-0 items-center gap-2">
				{props.result.role === undefined ? null : (
					<span class="text-xs text-fg-faint">
						{roleLabels[props.result.role]}
					</span>
				)}
				{deprecated() ? (
					<span class="tag-chip tag-warning">Deprecated</span>
				) : null}
			</span>
		</a>
	)
}

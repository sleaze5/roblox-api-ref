import { For } from "solid-js"
import type { SearchEngine } from "../../search/engine.ts"
import type { ResultGroup } from "../../search/result-info.ts"
import { ResultOption } from "./ResultOption.tsx"

export function ResultGroups(props: {
	engine: SearchEngine
	groups: ResultGroup[]
	word: string
}) {
	return (
		<For each={props.groups} keyed={false}>
			{(group) => (
				<div role="group" aria-label={group().title} class="pb-1">
					<p
						aria-hidden="true"
						class="px-2 pt-2 pb-1 text-xs font-medium text-fg-muted">
						{group().title}
					</p>
					<For each={group().results} keyed={false}>
						{(result, index) => (
							<ResultOption
								engine={props.engine}
								result={result()}
								index={group().start + index}
								word={group().typeName === undefined ? props.word : ""}
							/>
						)}
					</For>
				</div>
			)}
		</For>
	)
}

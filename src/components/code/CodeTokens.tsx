import type { CodeTone, CodeToken } from "../../api/code-token.ts"

const toneClasses: Record<CodeTone, string | undefined> = {
	self: "text-fg-muted",
	type: "text-code-type",
	constant: "text-code-constant",
	string: "text-code-string",
	function: "text-code-function",
	name: "font-medium text-fg-bright",
	parameter: undefined,
	punctuation: undefined,
	plain: undefined,
}

type TokenRun = {
	href?: string
	title?: string
	tokens: CodeToken[]
}

function mergeTones(tokens: CodeToken[]): CodeToken[] {
	const merged: CodeToken[] = []
	for (const part of tokens) {
		const last = merged.at(-1)
		if (
			last !== undefined &&
			toneClasses[last.tone] === toneClasses[part.tone] &&
			part.swatch === undefined
		) {
			merged[merged.length - 1] = { ...last, text: last.text + part.text }
		} else {
			merged.push(part)
		}
	}
	return merged
}

function runsOf(tokens: CodeToken[]): TokenRun[] {
	const runs: TokenRun[] = []
	for (const part of tokens) {
		const last = runs.at(-1)
		if (last !== undefined && last.href === part.href) {
			last.tokens.push(part)
			continue
		}
		const run: TokenRun = { tokens: [part] }
		if (part.href !== undefined) {
			run.href = part.href
		}
		if (part.title !== undefined) {
			run.title = part.title
		}
		runs.push(run)
	}
	for (const run of runs) {
		run.tokens = mergeTones(run.tokens)
	}
	return runs
}

function Swatch(props: { color: string }) {
	return (
		<span
			class="mr-1 inline-block size-3 rounded-xs border border-line-strong align-middle"
			style={{ "background-color": props.color }}
		/>
	)
}

function Tokens(props: { tokens: CodeToken[] }) {
	return (
		<>
			{props.tokens.map((part) => {
				const tone = toneClasses[part.tone]
				const swatch =
					part.swatch === undefined ? null : <Swatch color={part.swatch} />
				return tone === undefined && swatch === null ? (
					part.text
				) : (
					<span class={tone}>
						{swatch}
						{part.text}
					</span>
				)
			})}
		</>
	)
}

function Link(props: { run: TokenRun; href: string }) {
	const [only] = props.run.tokens
	if (
		props.run.tokens.length === 1 &&
		only !== undefined &&
		only.swatch === undefined
	) {
		return (
			<a
				href={props.href}
				data-tooltip={props.run.title}
				class={toneClasses[only.tone]}>
				{only.text}
			</a>
		)
	}
	return (
		<a href={props.href} data-tooltip={props.run.title}>
			<Tokens tokens={props.run.tokens} />
		</a>
	)
}

export function CodeTokens(props: { tokens: CodeToken[] }) {
	return (
		<>
			{runsOf(props.tokens).map((run) =>
				run.href === undefined ? (
					<Tokens tokens={run.tokens} />
				) : (
					<Link run={run} href={run.href} />
				),
			)}
		</>
	)
}

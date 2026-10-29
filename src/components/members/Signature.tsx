import { textLength } from "../../api/code-token.ts"
import { inlineTokens, type SignatureParts } from "../../api/signature.ts"
import { CodeTokens } from "../code/CodeTokens.tsx"

const wrapLength = 80

function InlineParameters(props: { parameters: SignatureParts["parameters"] }) {
	return (
		<>
			{(props.parameters ?? []).map((parameter, index) => (
				<>
					{index > 0 ? ", " : null}
					<CodeTokens tokens={parameter} />
				</>
			))}
		</>
	)
}

export function Signature(props: { parts: SignatureParts }) {
	const parameters = props.parts.parameters ?? []
	const wrapped =
		parameters.length >= 2 && textLength(inlineTokens(props.parts)) > wrapLength
	return (
		<code class="member-code">
			<CodeTokens tokens={props.parts.owner} />
			<wbr />
			<CodeTokens tokens={props.parts.head} />
			{wrapped ? (
				parameters.map((parameter) => (
					<span class="block pl-4">
						<CodeTokens tokens={parameter} />,
					</span>
				))
			) : (
				<>
					{props.parts.parameters === undefined ? null : <wbr />}
					<InlineParameters parameters={props.parts.parameters} />
				</>
			)}
			<CodeTokens tokens={props.parts.tail} />
		</code>
	)
}

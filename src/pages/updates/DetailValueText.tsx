import { classHref } from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import type { DetailValue } from "../../api/update-types.ts"

export function DetailValueText(props: { api: Api; value: DetailValue }) {
	switch (props.value.tone) {
		case "text":
			return <span>{props.value.text}</span>
		case "code":
			return <code>{props.value.text}</code>
		case "type":
			return props.api.classes[props.value.text] === undefined ? (
				<code class="text-code-type">{props.value.text}</code>
			) : (
				<code>
					<a href={classHref(props.value.text)} class="text-code-type">
						{props.value.text}
					</a>
				</code>
			)
	}
}

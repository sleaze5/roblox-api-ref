import { classHref, enumHref } from "../../api/routes.ts"
import type { Api } from "../../api/types.ts"
import type { OwnerChanges } from "../../api/update-types.ts"

function ownerHref(api: Api, owner: OwnerChanges): string | undefined {
	if (owner.kind === "Class") {
		return api.classes[owner.name] === undefined ? undefined : classHref(owner.name)
	}
	return api.enums[owner.name] === undefined ? undefined : enumHref(owner.name)
}

export function OwnerName(props: { api: Api; owner: OwnerChanges }) {
	const href = ownerHref(props.api, props.owner)
	return (
		<code class="min-w-0 truncate font-semibold text-code-type">
			{href === undefined ? (
				props.owner.name
			) : (
				<a href={href}>{props.owner.name}</a>
			)}
		</code>
	)
}

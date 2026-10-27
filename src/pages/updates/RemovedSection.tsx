import type { Removed } from "../../api/item-history.ts"
import { updateHref } from "../../api/routes.ts"
import type { Api, EnumItem, Member } from "../../api/types.ts"
import { changeAnchor, formatDate } from "../../api/update-text.ts"
import type { OwnerKind } from "../../api/update-types.ts"
import { Icon } from "../../components/Icon.tsx"
import { Signature } from "../../components/members/Signature.tsx"
import { ChangeMark } from "./ChangeMark.tsx"
import { changeParts } from "./change-signature.ts"

export function RemovedSection(props: {
	api: Api
	id: string
	title: string
	kind: OwnerKind
	owner: string
	removed: Removed<Member | EnumItem>[] | undefined
}) {
	const removed = props.removed ?? []
	return removed.length === 0 ? null : (
		<section
			id={props.id}
			aria-labelledby={`${props.id}-title`}
			class="flex flex-col gap-3">
			<h2
				id={`${props.id}-title`}
				class="flex items-center gap-2 text-lg font-semibold">
				<Icon name="clock-counter-clockwise" size={20} class="text-fg-muted" />
				{props.title}
				<span class="text-sm font-normal text-fg-faint">{removed.length}</span>
			</h2>
			<ul class="flex flex-col gap-3">
				{removed.map((item) => (
					<li class="flex flex-col gap-1">
						<div class="change-line">
							<ChangeMark status="Removed" label="Removed:" />
							<Signature
								parts={changeParts(
									props.api,
									props.owner,
									item.value,
									false,
								)}
							/>
						</div>
						<p class="flex flex-wrap items-baseline gap-x-2 change-indent text-fg-faint">
							Removed in
							<a
								href={`${updateHref(item.removed.version)}#${changeAnchor(props.kind, props.owner, item.value.name)}`}
								class="font-mono text-fg-muted hover:text-fg hover:underline">
								{item.removed.version}
							</a>
							<span class="text-xs">
								{formatDate(item.removed.published)}
							</span>
						</p>
					</li>
				))}
			</ul>
		</section>
	)
}

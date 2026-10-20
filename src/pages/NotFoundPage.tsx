import { classesHref } from "../api/routes.ts"
import type { Api } from "../api/types.ts"
import { Icon } from "../components/Icon.tsx"
import { rootCrumbs } from "../components/page/crumbs.ts"
import { PageHeader } from "../components/page/PageHeader.tsx"
import { PageLead } from "../components/page/PageLead.tsx"
import { Shell } from "../layout/Shell.tsx"

export function NotFoundPage(props: { api: Api }) {
	return (
		<Shell release={props.api}>
			<PageHeader crumbs={rootCrumbs} icon="warning" title="Page not found">
				<PageLead>
					This page does not exist in Studio {props.api.version}. Search for
					the class or enum, or browse the{" "}
					<a
						href={classesHref()}
						class="text-fg underline decoration-line-strong hover:decoration-fg">
						class tree
					</a>
					.
				</PageLead>
				<button type="button" data-search-open="" class="button w-fit">
					<Icon name="magnifying-glass" size={14} />
					Search the API
				</button>
			</PageHeader>
		</Shell>
	)
}

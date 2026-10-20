export type Crumb = {
	label: string
	href: string
	code?: boolean
}

export function Breadcrumbs(props: { crumbs: Crumb[] }) {
	return (
		<nav aria-label="Breadcrumb">
			<ol class="flex flex-wrap items-center gap-x-1 text-fg-muted">
				{props.crumbs.map((crumb, index) => (
					<li class="flex items-center gap-1">
						{index > 0 ? (
							<span aria-hidden="true" class="text-fg-faint">
								›
							</span>
						) : null}
						<a
							href={crumb.href}
							class={[
								"hover:underline",
								crumb.code
									? "font-mono text-code-type"
									: "motion-colors hover:text-fg",
							]}>
							{crumb.label}
						</a>
					</li>
				))}
			</ol>
		</nav>
	)
}

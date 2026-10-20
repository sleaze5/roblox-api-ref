import { sectionLinks, type Section } from "./section-links.ts"

export function DrawerSections(props: { section: Section | undefined }) {
	return (
		<ul
			aria-label="Sections"
			class="mb-3 flex flex-col border-b border-line pb-3 md:hidden">
			{sectionLinks.map((link) => (
				<li>
					<a
						href={link.href}
						aria-current={
							props.section === link.section ? "page" : undefined
						}
						class={[
							"flex h-10 items-center rounded-sm px-2 font-medium motion-colors hover:bg-hover hover:text-fg",
							props.section === link.section
								? "bg-active text-fg-bright"
								: "text-fg-muted",
						]}>
						{link.label}
					</a>
				</li>
			))}
		</ul>
	)
}

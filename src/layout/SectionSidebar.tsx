import type { JSX } from "@solidjs/web"
import { ListFilter } from "../components/filters/ListFilter.tsx"

const scrollToCurrent =
	"{const n=document.getElementById('sidebar'),c=n&&n.querySelector('[data-tree] [aria-current=page]');if(c)n.scrollTop=c.getBoundingClientRect().top-n.getBoundingClientRect().top-n.clientHeight/3}"

export function SectionSidebar(props: {
	label: string
	target: string
	children: JSX.Element
}) {
	return (
		<>
			<ListFilter
				label={props.label}
				target={props.target}
				param="nav"
				placement="sidebar"
			/>
			{props.children}
		</>
	)
}

export function SidebarScroll() {
	return <script innerHTML={scrollToCurrent} />
}

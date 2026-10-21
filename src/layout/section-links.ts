import { classesHref, enumsHref, indexHref, updatesHref } from "../api/routes.ts"

export type Section = "home" | "classes" | "enums" | "index" | "updates"

export type SectionLink = {
	section: Section
	label: string
	href: string
}

export const sectionLinks: SectionLink[] = [
	{ section: "classes", label: "Classes", href: classesHref() },
	{ section: "enums", label: "Enums", href: enumsHref() },
	{ section: "index", label: "Index", href: indexHref() },
	{ section: "updates", label: "Updates", href: updatesHref() },
]

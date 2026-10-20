import { classesHref, enumsHref, indexHref } from "../api/routes.ts"

export type Section = "home" | "classes" | "enums" | "index"

export type SectionLink = {
	section: Section
	label: string
	href: string
}

export const sectionLinks: SectionLink[] = [
	{ section: "classes", label: "Classes", href: classesHref() },
	{ section: "enums", label: "Enums", href: enumsHref() },
	{ section: "index", label: "Index", href: indexHref() },
]
